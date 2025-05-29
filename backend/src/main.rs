use std::{collections::{hash_map::Entry, HashMap}, sync::Arc, time::{Duration, Instant}};

use axum::{
    routing::{any, delete, get, post}, Router
};
use chats::Message;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::{net::TcpListener, sync::{watch::{Receiver, Sender}, Mutex}};
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;
use webauthn_rs::prelude::{PasskeyAuthentication, PasskeyRegistration, Url};
mod auth;
mod chats;
mod stats;

#[derive(Clone)]
pub enum Update{
    Empty,
    MessageAdded(Message)
}

pub struct Synchronizer{
    map: HashMap<Uuid, (Sender<Update>, Receiver<Update>)>
} 

impl Synchronizer{
    pub fn new() -> Synchronizer{
        Synchronizer{
            map: HashMap::<_, _>::new()
        }
    }


    pub fn get_receiver(&mut self, chat_id: Uuid) -> Receiver<Update>{
        let (_, receiver) = match self.map.entry(chat_id) {
            Entry::Occupied(o) => {
                    o.into_mut()
            },
            Entry::Vacant(v) => {
                v.insert(tokio::sync::watch::channel(Update::Empty))
            }
        };
        receiver.clone()
    }

    pub fn post_message(&mut self, chat_id: Uuid, message: Message){
        if let Some((sender, _)) = self.map.get_mut(&chat_id) {
            if let Err(e) = sender.send(Update::MessageAdded(message)){
                println!("Sync send error: {}", e);
            }
        }
    }
}

#[derive(Clone)]
pub struct State{
    db_pool: Pool<Postgres>,
    synchronizer: Arc<Mutex<Synchronizer>>,
    webauthn: Arc<webauthn_rs::Webauthn>,
    ongoing_passkey_registrations: Arc<Mutex<[Option<(Uuid, Instant, PasskeyRegistration)>; 5]>>,
    ongoing_passkey_logins: Arc<Mutex<[Option<(Uuid, Instant, PasskeyAuthentication)>; 10]>>
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


    let synchronizer = Synchronizer::new();
    let async_hronizer = Arc::new(Mutex::new(synchronizer));

    //let auth_middleware = axum::middleware::from_fn(auth::auth_middleware);

    let rp_id = "localhost";
    let rp_origin:Url = "http://localhost:3000".parse().unwrap();
    let webauthn = 
        webauthn_rs::WebauthnBuilder::new(rp_id, &rp_origin)
        .expect("Invalid webauthn config")
        .rp_name("ChatReview")
        .build().expect("Failed to build webauthn config");

    let state = State{
        db_pool,
        synchronizer: async_hronizer,
        webauthn: Arc::new(webauthn),
        ongoing_passkey_registrations: Arc::new(Mutex::new([const { None }; 5])),
        ongoing_passkey_logins: Arc::new(Mutex::new( [const { None }; 10]))
    };

    // compose the routes
    let app = Router::new()
        .route("/", get(|| async { "Connection to chatreview API appears to be working!" }))
        .route("/mychats/", get(chats::get_my_chats))
        .route("/random_chat/", get(chats::get_random_chat))
        .route("/message/", post(chats::post_message))
        .route("/rating/", post(chats::post_rating))
        .route("/chat/random/", post(chats::create_chat_with_random))
        .route("/chat/fromMyPerspective/{chatId}", get(chats::get_chat_by_id_from_user_perspective))
        .route("/chat/{chatId}", get(chats::get_chat_by_id))
        .route("/chat/", post(chats::create_chat))

        .route("/comment/", post(chats::post_comment))
        .route("/comment/forMessage/{messageId}", get(chats::get_comments_for_message))

        .route("/profile/", delete(auth::delete_account_handler))
        //.route_layer(auth_middleware)
        .route("/register/", post(auth::register_handler))
        .route("/login/", post(auth::login_handler))
        .route("/loginWithPasskey/", post(auth::passkey_login_handler))
        .route("/loginWithPasskey/complete/", post(auth::complete_passkey_login))
        .route("/registerPasskey/", post(auth::register_webauthn))
        .route("/registerPasskey/complete/", post(auth::complete_register_webauthn))

        .route("/stats/", get(stats::get_stats))
        .route("/ws/{chatId}", any(chats::websocket_chat_updates))

        .layer(
            CorsLayer::new()
                .allow_origin([
                    "http://192.168.1.200:3000".parse().unwrap(),
                    "http://localhost:3000".parse().unwrap(),
                    "https://zenonet.de".parse().unwrap(),
                ])
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(state);

    //serve the application
    axum::serve(listener, app)
        .await
        .expect("Error serving application");
}