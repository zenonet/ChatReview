use std::time::Duration;

use axum::{
    routing::{delete, get, post}, Router
};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
mod auth;
mod chats;
mod stats;

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
        .route("/", get(|| async { "Connection to chatreview API appears to be working!" }))
        .route("/mychats/", get(chats::get_my_chats))
        .route("/random_chat/", get(chats::get_random_chat))
        .route("/message/", post(chats::post_message))
        .route("/rating/", post(chats::post_rating))
        .route("/chat/random/", post(chats::create_chat_with_random))
        .route("/chat/{chatId}", get(chats::get_chat_by_id))
        .route("/chat/", post(chats::create_chat))

        .route("/comment/", post(chats::post_comment))
        .route("/comment/forMessage/{messageId}", get(chats::get_comments_for_message))

        .route("/profile/", delete(auth::delete_account_handler))
        //.route_layer(auth_middleware)
        .route("/register/", post(auth::register_handler))
        .route("/login/", post(auth::login_handler))

        .route("/stats/", get(stats::get_stats))
        //.route("/tasks/:task_id", patch(update_task).delete(delete_task))
        .layer(
            CorsLayer::new()
                .allow_origin([
                    #[cfg(debug_assertions)]
                    "http://localhost:3000".parse().unwrap(),
                    "https://zenonet.de".parse().unwrap(),
                ])
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(db_pool);

    //serve the application
    axum::serve(listener, app)
        .await
        .expect("Error serving application");
}