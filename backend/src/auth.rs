use std::borrow::Cow;
use std::sync::{Arc};
use std::time::Instant;

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use chrono::{Duration, Utc};
use axum::{extract::State, http::StatusCode};
use axum::Json;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;

use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use tokio::sync::Mutex;
use uuid::Uuid;
use webauthn_rs::prelude::{Passkey, PublicKeyCredential, RegisterPublicKeyCredential, RequestChallengeResponse, WebauthnError};


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
    async fn from_request_parts(parts: &mut Parts, state: &S,) -> Result<Self,Self::Rejection> {
        let _ = state;
        if let Some(token) = parts.headers.get("Authorization") {
            let token = token.to_str().unwrap();
            return validate_token(token);
        }
        
        return Err((
            StatusCode::UNAUTHORIZED,
            String::from("The route you called requires authorization")
        ))
    }
}

pub(crate) fn validate_token(token: &str) -> Result<Claims, (StatusCode, String)> {
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

    return Err((
        StatusCode::UNAUTHORIZED,
        String::from("Invalid token")
    ))
}


pub struct UserRow{
    username: String,
    password: String,
    id: Uuid,
}


pub async fn delete_account_handler(
    user: Claims,
    State(state): State<crate::State>
) -> StatusCode{

    let res = sqlx::query!("DELETE FROM users WHERE id=$1", user.id)
        .execute(&state.db_pool).await;

    if let Ok(res) = res{
        if res.rows_affected() == 1{
            return StatusCode::OK;
        }
    }
    StatusCode::INTERNAL_SERVER_ERROR
}


#[derive(Deserialize)]
pub struct RegisterRequestBody{
    username: String,
    password: String,
}

pub async fn register_handler(
    State(state): State<crate::State>,
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
    ).execute(&state.db_pool).await
    .map_err(|e|{
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string()
        )
    })?;

    Ok((
        StatusCode::CREATED,
        json!({
            "token": generate_jwt(user_row.id, user_row.username).unwrap()
        }).to_string()
    ))
}



/// .
///
/// # Panics
///
/// Panics if .
async fn put_into_slot<A, B, O, const SIZE:usize>(buffer: &Arc<Mutex<[Option<(A, O, B)>;SIZE]>>, value: (A, O, B)) 
where O : Ord + Copy {
    let mut locked = buffer.lock().await;

    // Find an empty slot
    let slot = locked.iter_mut().find(|reg|{ reg.is_none() });

    // if a slot is empty, use it
    let slot = if let Some(slot) = slot{
        slot
    }else{
        println!("Passkey registration: No empty slot available, using oldest one");
        // If no slot is empty, clear the oldest one

        locked.sort_by_key(|reg|{
            if let Some(reg) = reg {
                return reg.1;
            }
            unreachable!("Registration slots should all be some here!")
        });

        locked.get_mut(0).expect("")
    };

    *slot = Some(value);
}


pub async fn register_webauthn(
    user: Claims,
    State(state): State<crate::State>,
) -> Result<impl IntoResponse, (StatusCode, String)>{
    // TODO: Make sure that the user doesn't re-register the passkey
    let res = state.webauthn.start_passkey_registration(user.id, &user.username, &user.username, None);
    
    let (resp, registration) = res.unwrap();

    // Store the registration in memory until the user completes the registration process
    put_into_slot(&state.ongoing_passkey_registrations, (user.id, Instant::now(), registration)).await;

    Ok(json!(resp).to_string())
}

pub async fn complete_register_webauthn(
    user: Claims,
    State(state): State<crate::State>,
    Json(reg): Json<RegisterPublicKeyCredential>
) -> Result<impl IntoResponse, (StatusCode, String)>{

    // find registration
    let registration = {
        let mut regs = state.ongoing_passkey_registrations.lock().await;
        let registration = regs.iter_mut().find(|reg|{
            reg.as_ref().is_some_and(|reg|{
                reg.0 == user.id
            })
        });

        if let Some(registration) = registration{
            // Move the registration out of app state
            std::mem::replace(registration, None).unwrap()
        }else{
            return Err((
                StatusCode::BAD_REQUEST,
                String::from("Couldn't find ongoing registration")
            ));
        }
    };

    let res = state.webauthn.finish_passkey_registration(&reg, &registration.2);


    let passkey = res.map_err(|e|{(
        StatusCode::INTERNAL_SERVER_ERROR, 
        format!("Failed to create passkey: {}", e)
    )})?;

    // save passkey to databse
    let id = Uuid::new_v4();
    sqlx::query!("INSERT INTO passkeys (id,user_id,name,data,creation_date) VALUES ($1,$2,$3,$4,$5)",
        id,
        user.id,
        format!("{}'s passkey", user.username), // TODO
        serde_json::to_string(&passkey).unwrap(),
        Utc::now()
    ).execute(&state.db_pool).await.map_err(|e|{(
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Failed to save public key: {}", e)
    )})?;

    Ok(())
}

#[derive(Deserialize)]
pub struct PasskeyLoginRequest{
    pub username: String
}

pub async fn passkey_login_handler(
    State(state): State<crate::State>,
    Json(body): Json<PasskeyLoginRequest>
) -> Result<impl IntoResponse, (StatusCode, String)>{

    // Get token from db
    let res = 
        sqlx::query!("SELECT passkeys.* FROM passkeys JOIN users ON users.id = passkeys.user_id WHERE users.username = $1 LIMIT 1", body.username)
        .fetch_all(&state.db_pool).await;

    let passkey_rows = match res {
        Ok(r) => r,
        Err(e) => return Err((
            StatusCode::BAD_REQUEST,
            String::from("Couldn't find passkey")
        ))
    };

    let passkey = passkey_rows.iter().map(|row|{
        serde_json::from_str(&row.data)
    })
        .collect::<Result<Vec<Passkey>, _>>()
        .map_err(|_|{(
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Failed to deserialize public key for passkey authentication")
    )})?;

    let res = state.webauthn.start_passkey_authentication(&passkey)
    .map_err(|e|{(
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Failed to create login challenge: {}", e)
    )})?;


    let user_id = passkey_rows.first().unwrap().user_id;

    put_into_slot(&state.ongoing_passkey_logins, (user_id, Instant::now(), res.1)).await;

    #[derive(Serialize)]
    struct Resp{
        user_id: Uuid,
        challenge: RequestChallengeResponse
    }

    Ok(Json(json!({
        "userId": user_id,
        "challenge": res.0
    })))
}


pub async fn complete_passkey_login(
    State(state): State<crate::State>,
    headers: HeaderMap,
    Json(credential): Json<PublicKeyCredential>
) -> Result<impl IntoResponse, (StatusCode, String)>{

    let Some(user_id_header) = headers.get("userId") else{
        return Err((
            StatusCode::BAD_REQUEST,
            String::from("Expected userId header")
        ));
    };


    let user_id: Uuid =
    if let Ok(header_str) = user_id_header.to_str(){
        if let Ok(id) = Uuid::try_parse(header_str){
            id
        }else{
            return Err((
                StatusCode::BAD_REQUEST,
                String::from("Invalid userId header")
            ))
        }
    }else{
        return Err((
            StatusCode::BAD_REQUEST,
            String::from("Invalid userId header")
        ))
    };
    
    

    let logins = state.ongoing_passkey_logins.lock().await;
    let Some(login) = logins.iter().find(|l|{
        l.as_ref().is_some_and(|l|{
            l.0 == user_id
        })
    }) else{
        return Err((
            StatusCode::UNAUTHORIZED,
            String::from("No login challenge present for this user.")
        ));
    };
    
    let Some(login) = login else{
        unreachable!();
    };

    let res = state.webauthn.finish_passkey_authentication(&credential, &login.2).map_err(|e|{(
        StatusCode::UNAUTHORIZED,
        format!("Signature validation failed")
    )})?;


    let username = sqlx::query_scalar!("SELECT username FROM users WHERE id=$1", user_id)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e|{(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch user data: {}", e)
        )})?;

    let token = generate_jwt(user_id, username).unwrap();
    Ok((
        StatusCode::OK,
        json!({
            "token": token
        }).to_string()
    ))
}


#[derive(Deserialize)]
pub struct LoginRequestBody{
    username: String,
    password: String,
}

pub async fn login_handler(
    State(state): State<crate::State>,
    Json(body): Json<LoginRequestBody>
) -> Result<(StatusCode, String), (StatusCode, String)>{
    let user_row = sqlx::query_as!(UserRow, "SELECT username, password, id FROM users WHERE username=$1", body.username)
        .fetch_optional(&state.db_pool)
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
            let token = generate_jwt(user_row.id, user_row.username).unwrap();
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


fn generate_jwt(user_id: Uuid, username: String) -> Result<String, jsonwebtoken::errors::Error>{
    jsonwebtoken::encode(
        &Header::default(),
        &Claims{
            id: user_id,
            username: username,
            exp: chrono::Utc::now().checked_add_signed(Duration::minutes(120)).unwrap().timestamp() as usize
        },
        &EncodingKey::from_base64_secret(&std::env::var("JWT_SECRET").unwrap()).unwrap()
    )
}