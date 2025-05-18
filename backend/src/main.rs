use std::{os::linux, time::Duration};

use auth::Claims;
use axum::{
    Json, Router,
    body::Body,
    extract::State,
    handler::Layered,
    http::StatusCode,
    middleware::FromFnLayer,
    response::IntoResponse,
    routing::{Route, get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{PgPool, postgres::PgPoolOptions, prelude::FromRow};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

mod auth;

#[derive(Serialize, Deserialize, FromRow)]
struct ChatRow {
    id: Uuid,
    name: Option<String>,
    description: Option<String>,
    owner_id: Uuid,
}

#[derive(Serialize)]
struct Chat {
    id: Uuid,
    name: String,
    description: Option<String>,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, FromRow)]
struct MessageRow {
    id: Uuid,
    content: Option<String>,

    #[serde(rename = "isOwn")]
    is_own: Option<bool>,

    #[serde(rename = "chatId")]
    chat_id: Option<Uuid>,
    index: i32,
}

#[derive(Serialize)]
struct Message {
    #[serde(default)]
    id: Uuid,
    content: String,
    #[serde(rename = "isOwn")]
    is_own: bool,
}

#[tokio::main]
async fn main() {
    //expose environment variables from .env file
    dotenvy::dotenv().expect("Unable to access .env file");

    //set variables from enviroment variables
    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in env file");

    //create our database pool
    let db_pool = PgPoolOptions::new()
        .max_connections(64)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await
        .expect("can't connect to database");

    //create our tcp listener
    let listener = TcpListener::bind(server_address)
        .await
        .expect("Could not create tcp listener");

    println!("listening on http://{}", listener.local_addr().unwrap());

    //let auth_middleware = axum::middleware::from_fn(auth::auth_middleware);

    // compose the routes
    let app = Router::new()
        .route("/", get(|| async { "Hello world" }))
        .route("/chats/", get(get_chats) /* .post(create_task) */)
        .route("/random_chat/", get(get_random_chat))
        .route("/message/", post(post_message))
        .route("/chat/", post(create_chat))
        //.route_layer(auth_middleware)
        .route("/register/", post(auth::register_handler))
        .route("/login/", post(auth::login_handler))
        //.route("/tasks/:task_id", patch(update_task).delete(delete_task))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any) // TODO
                .allow_headers(Any),
        )
        .with_state(db_pool);

    //serve the application
    axum::serve(listener, app)
        .await
        .expect("Error serving application");
}

async fn get_chats(
    State(db_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let rows = sqlx::query_as!(ChatRow, "SELECT * FROM chats")
        .fetch_all(&db_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success": false, "message": e.to_string()}).to_string(),
            )
        })?;

    Ok((StatusCode::OK, json!(rows).to_string()))
}

// TODO: Investigate how to return objects directly and let them be serialized by axum
async fn get_random_chat(
    State(db_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let chat_row = sqlx::query_as!(ChatRow, "SELECT * FROM chats ORDER BY RANDOM() LIMIT 1")
        .fetch_all(&db_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success": false, "message": e.to_string()}).to_string(),
            )
        })?
        .into_iter()
        .next();

    let Some(chat_row) = chat_row else {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("No chat available"),
        ));
    };

    // TODO: Maybe do a join here? Idk, that would mean that a lot of would have to be sent twice

    // Also fetch the messages of the chat
    let message_rows = sqlx::query_as!(
        MessageRow,
        "SELECT id, content, chat_id, index, is_own FROM chat_messages WHERE chat_id=$1",
        chat_row.id
    )
    .fetch_all(&db_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "message": e.to_string()}).to_string(),
        )
    })?;

    let chat = Chat {
        id: chat_row.id,
        name: chat_row.name.unwrap(),
        description: chat_row.description,
        messages: message_rows
            .into_iter()
            .map(|row| Message {
                content: row.content.unwrap(),
                id: row.id,
                is_own: row.is_own.unwrap_or_else(|| true),
            })
            .collect(),
    };

    Ok((StatusCode::OK, json!(chat).to_string()))
}


#[derive(Deserialize)]
struct NewChat{
    name:String,
    description: Option<String>,
}

async fn create_chat(
    user: Claims,
    State(db_pool): State<PgPool>,
    Json(chat): Json<NewChat>,
) -> Result<(StatusCode, String), (StatusCode, String)> {

    let chat_id = Uuid::new_v4();

    if sqlx::query!(
        "INSERT INTO chats (id, name, description, owner_id) VALUES ($1,$2,$3,$4)",
        chat_id,
        chat.name,
        chat.description,
        user.id
    )
    .execute(&db_pool)
    .await
    .is_ok()
    {
        Ok((StatusCode::CREATED, chat_id.to_string()))
    } else {
        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Failed to create chat"),
        ))
    }
}


#[derive(Deserialize)]
struct NewMessage{
    #[serde(alias = "chatId")]
    chat_id: Uuid,
    content: String,
    #[serde(alias = "isOwn")]
    is_own: bool,
    index: i32
}

async fn post_message(
    user: Claims,
    State(db_pool): State<PgPool>,
    Json(mut message): Json<NewMessage>,
) -> Result<StatusCode, (StatusCode, String)> {
    // Check if the specified chat belongs to the user
    let _ = sqlx::query!(
        "SELECT owner_Id FROM chats WHERE id=$1 AND owner_id=$2 LIMIT 1",
        message.chat_id,
        user.id
    )
    .fetch_one(&db_pool)
    .await
    .map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            String::from("The chat doesn't exist or doesn't belong to you"),
        )
    })?;

    // At this point, the query would've thrown an error if the chat doesn't exist or doesn't belong to the user

    let message_id = Uuid::new_v4();

    let query = sqlx::query!(
        "INSERT INTO chat_messages (id, content, chat_id, index, is_own) VALUES ($1,$2,$3,$4,$5)",
        message_id,
        message.content,
        message.chat_id,
        message.index,
        message.is_own
    );

    if let Ok(_) = query.execute(&db_pool).await {
        Ok(StatusCode::CREATED)
    } else {
        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("creating message failed"),
        ))
    }
}
