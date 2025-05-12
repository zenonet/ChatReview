use std::time::Duration;

use axum::{extract::State, http::StatusCode, routing::get, Router};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{postgres::PgPoolOptions, prelude::FromRow, PgPool};
use tokio::net::TcpListener;
use uuid::Uuid;


#[derive(Serialize, Deserialize, FromRow)]
struct Chat{
  id: Uuid,
  name: Option<String>,
  description: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow)]
struct Message{
  id: Uuid,
  content: String,
  own: bool,
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
    .route("/chats/", get(get_chats)/* .post(create_task) */)
    .route("/random_chat/", get(get_random_chat))
    //.route("/tasks/:task_id", patch(update_task).delete(delete_task))
    .with_state(db_pool);

  //serve the application
  axum::serve(listener, app)
    .await
    .expect("Error serving application");
}


async fn get_chats(
  State(db_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
  let rows = sqlx::query_as!(Chat, "SELECT * FROM chats")
    .fetch_all(&db_pool)
    .await
    .map_err(|e| {
      (
        StatusCode::INTERNAL_SERVER_ERROR,
        json!({"success": false, "message": e.to_string()}).to_string(),
      )
    })?;

  Ok((
    StatusCode::OK,
    json!({"success": true, "data": rows}).to_string(),
  ))
}

async fn get_random_chat(
  State(db_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
  let chat = sqlx::query_as!(Chat, "SELECT * FROM chats ORDER BY RANDOM() LIMIT 1")
    .fetch_all(&db_pool)
    .await
    .map_err(|e| {
      (
        StatusCode::INTERNAL_SERVER_ERROR,
        json!({"success": false, "message": e.to_string()}).to_string(),
      )
    })?
    .into_iter().next().unwrap();

  Ok((
    StatusCode::OK,
    //json!({"success": true, "data": rows}).to_string(),
    json!(chat).to_string()
  ))
}