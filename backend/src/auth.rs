use axum::body::Body;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use chrono::{DateTime, Duration, TimeDelta};
use axum::{extract::State, http::StatusCode};
use axum::{extract, Json};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{PgPool, Type};

use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use uuid::Uuid;


#[derive(Serialize, Deserialize, Clone)]
pub struct Claims{
    pub id: Uuid,
    pub username: String,
    pub exp: usize
}

macro_rules! invalid_login_error {
    () => {
        Err((
            StatusCode::UNAUTHORIZED,
            json!({
                "error": "Username or password incorrect",
            }).to_string()
        ))
    };
}

impl<S> FromRequestParts<S> for Claims
where S: Send + Sync{
    #[doc = " If the extractor fails it\'ll use this \"rejection\" type. A rejection is"]
#[doc = " a kind of error that can be converted into a response."]
type Rejection = (StatusCode, String);

    #[doc = " Perform the extraction."]
    async fn from_request_parts(parts: &mut Parts,state: &S,) -> Result<Self,Self::Rejection> {
        if let Some(token) = parts.headers.get("Authorization") {
            let token = token.to_str().unwrap();
            if !token.starts_with("Bearer "){
                return Err((
                    StatusCode::UNAUTHORIZED,
                    String::from("The provided token was in the wrong format")
                ));
            }
            let token: &str = &token[7..];
            if let Ok(token_data) = jsonwebtoken::decode::<Claims>(
                token,
                &DecodingKey::from_base64_secret(&std::env::var("JWT_SECRET").unwrap()).unwrap(),
                &Validation::default()
            ){
                return Ok(token_data.claims)
            }
        }
        return Err((
            StatusCode::UNAUTHORIZED,
            String::from("Invalid token")
        ))
        
    }
}


pub struct UserRow{
    username: String,
    password: String,
    id: Uuid,
}


#[derive(Deserialize)]
pub struct RegisterRequestBody{
    username: String,
    password: String,
}

pub async fn register_handler(
    State(db_pool): State<PgPool>,
    Json(body):  Json<RegisterRequestBody>
) -> Result<(StatusCode, String), (StatusCode, String)>{
    let salt = SaltString::generate(&mut OsRng);

    // TODO: Check default configuration
    let argon2 = Argon2::default();

    let hash = argon2
        .hash_password(body.password.as_bytes(), &salt).unwrap().to_string();

    let user_row = UserRow{
        id: Uuid::new_v4(),
        username: body.username,
        password: hash,
    };

    sqlx::query!(
        "INSERT INTO users (id,username,password, registration_date) VALUES ($1,$2,$3, $4)",
        user_row.id,
        user_row.username,
        user_row.password,
        chrono::Utc::now()
    ).execute(&db_pool).await
    .map_err(|e|{
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string()
        )
    })?;

    Ok((
        StatusCode::CREATED,
        json!({
            "token": generate_jwt(user_row).unwrap()
        }).to_string()
    ))
}





#[derive(Deserialize)]
pub struct LoginRequestBody{
    username: String,
    password: String,
}

pub async fn login_handler(
    State(db_pool): State<PgPool>,
    Json(body): Json<LoginRequestBody>
) -> Result<(StatusCode, String), (StatusCode, String)>{
    let user_row = sqlx::query_as!(UserRow, "SELECT username, password, id FROM users WHERE username=$1", body.username)
        .fetch_optional(&db_pool)
        .await
        .map_err(|e|{(
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({
                "error": e.to_string()
            }).to_string()
        )})?;

    let Some(user_row) = user_row else {
        return invalid_login_error!();
    };

    let hash = PasswordHash::new(&user_row.password).unwrap();
    if Argon2::default()
        .verify_password(body.password.as_bytes(), &hash).is_ok() {
            // Valid password was entered
            let token = generate_jwt(user_row).unwrap();
            Ok((
                StatusCode::OK,
                json!({
                    "token": token
                }).to_string()
            ))
    }else{
        invalid_login_error!()
    }
}

fn generate_jwt(user: UserRow) -> Result<String, jsonwebtoken::errors::Error>{
    jsonwebtoken::encode(
        &Header::default(),
        &Claims{
            id: user.id,
            username: user.username,
            exp: chrono::Utc::now().checked_add_signed(Duration::minutes(120)).unwrap().timestamp() as usize
        },
        &EncodingKey::from_base64_secret(&std::env::var("JWT_SECRET").unwrap()).unwrap()
    )
}