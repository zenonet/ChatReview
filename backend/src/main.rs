use std::{os::linux, time::Duration};

use axum::{body::Body, extract::State, http::StatusCode, routing::{get, post}, Json, Router};
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
}


#[derive(Serialize)]
struct Chat {
    id: Uuid,
    name: String,
    description: Option<String>,
    messages: Vec<Message>
}

#[derive(Serialize, Deserialize, FromRow)]
struct MessageRow {
    id: Uuid,
    content: Option<String>,  
    is_own: Option<bool>,

    chat_id: Option<Uuid>,
}

#[derive(Serialize)]
struct Message{
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

    // compose the routes
    let app = Router::new()
        .route("/", get(|| async { "Hello world" }))
        .route("/chats/", get(get_chats) /* .post(create_task) */)
        .route("/random_chat/", get(get_random_chat))
        .route("/register/", post(auth::register_handler))
        .route("/login/", post(auth::login_handler))
        //.route("/tasks/:task_id", patch(update_task).delete(delete_task))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any) // TODO
                .allow_headers(Any)
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
        .next()
        .unwrap();

    // TODO: Maybe do a join here? Idk, that would mean that a lot of would have to be sent twice
    
    // Also fetch the messages of the chat
    let message_rows = sqlx::query_as!(MessageRow, "SELECT * FROM chat_messages WHERE chat_id=$1", chat_row.id)
        .fetch_all(&db_pool)
        .await
        .map_err(|e|{
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success": false, "message": e.to_string()}).to_string()
            )
        })?;

    let chat = Chat{
        id: chat_row.id,
        name: chat_row.name.unwrap(),
        description: chat_row.description,
        messages: message_rows.into_iter().map(|row|{
            Message{
                content: row.content.unwrap(),
                id: row.id,
                is_own: row.is_own.unwrap_or_else(||{true})
            }
        }).collect()
    };

    Ok((
        StatusCode::OK,
        json!(chat).to_string(),
    ))
}