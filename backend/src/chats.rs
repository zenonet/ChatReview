use std::time::Duration;

use crate::auth;
use crate::Synchronizer;
use crate::Update;
use crate::auth::Claims;
use axum::body::Body;
use axum::http::Response;
use axum::Json;
use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::extract::WebSocketUpgrade;
use axum::extract::ws;
use axum::extract::ws::CloseFrame;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{FromRow, PgPool};
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
struct ChatRow {
    id: Uuid,
    name: Option<String>,
    description: Option<String>,
    user_id_a: Uuid,
    user_id_b: Option<Uuid>,
}

#[derive(Serialize)]
pub struct Chat {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub messages: Vec<Message>,
    pub is_from_perspective_a: bool,
}

struct MessageRow {
    id: Uuid,
    content: Option<String>,

    is_owned_by_a: Option<bool>,

    chat_id: Option<Uuid>,
    index: i32,

    avg_rating: Option<f64>,
}

#[derive(Serialize, Clone)]
pub struct Message {
    #[serde(default)]
    pub id: Uuid,
    pub content: String,
    #[serde(rename = "isOwn")]
    pub is_own: bool,

    pub avg_rating: f64,
}

pub(crate) async fn get_my_chats(
    user: Claims,
    State(state): State<crate::State>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let rows = sqlx::query_as!(
        ChatRow,
        "SELECT * FROM chats WHERE (user_id_a=$1 OR user_id_b=$1)",
        user.id
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "message": e.to_string()}).to_string(),
        )
    })?;

    Ok((StatusCode::OK, json!(rows).to_string()))
}

#[derive(Deserialize)]
pub(crate) struct NewRating {
    pub(crate) value: f64,
    #[serde(alias = "messageId")]
    pub(crate) message_id: Uuid,
}

pub(crate) async fn post_rating(
    user: Claims,
    State(state): State<crate::State>,
    Json(rating): Json<NewRating>,
) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query!("INSERT INTO message_ratings (message_id, owner_id, value, changed) VALUES ($1,$2,$3,$4) ON CONFLICT (message_id,owner_id) DO UPDATE SET value = $3, changed = $4",
            rating.message_id,
            user.id,
            rating.value,
            chrono::Utc::now()
        ).execute(&state.db_pool).await.map_err(|e|{
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                e.to_string()
            )
        })?;

    Ok(StatusCode::CREATED)
}

pub(crate) async fn get_messages_from_chat_id(
    id: Uuid,
    db_pool: &PgPool,
) -> Result<Vec<Message>, sqlx::error::Error> {
    let message_rows = sqlx::query_as!(
        MessageRow,
        r#"
SELECT
    m.id,
    m.content,
    m.chat_id,
    m.index,
    m.is_owned_by_a,
    AVG(r.value) AS avg_rating
FROM
    chat_messages m
LEFT JOIN
    message_ratings r ON m.id = r.message_id
WHERE m.chat_id = $1
GROUP BY m.id
ORDER BY m.index"#,
        id
    )
    .fetch_all(db_pool)
    .await?;

    Ok(message_rows
        .into_iter()
        .map(|row| Message {
            content: row.content.unwrap(),
            id: row.id,
            is_own: row.is_owned_by_a.unwrap_or_else(|| true),
            avg_rating: row.avg_rating.unwrap_or(0f64),
        })
        .collect::<Vec<Message>>())
}

// TODO: Investigate how to return objects directly and let them be serialized by axum
pub(crate) async fn get_random_chat(
    State(state): State<crate::State>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let chat_row = sqlx::query_as!(ChatRow, "SELECT * FROM chats ORDER BY RANDOM() LIMIT 1")
        .fetch_all(&state.db_pool)
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
    let messages = get_messages_from_chat_id(chat_row.id, &state.db_pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let chat = Chat {
        id: chat_row.id,
        name: chat_row.name.unwrap(),
        description: chat_row.description,
        messages,
        is_from_perspective_a: true,
    };

    Ok((StatusCode::OK, json!(chat).to_string()))
}

async fn get_chat_row_from_id(id: Uuid, db_pool: &PgPool) -> Result<ChatRow, sqlx::Error> {
    sqlx::query_as!(ChatRow, "SELECT * FROM chats WHERE id=$1", id,)
        .fetch_one(db_pool)
        .await
}

/// Loads a chat object from the database based on a chat id
///
/// # Errors
///
/// This function will return an error if the sql query fails or the chat doesn't exist
async fn load_chat_from_db(id: Uuid, db_pool: &PgPool) -> Result<Chat, sqlx::Error> {
    let chat = get_chat_row_from_id(id, db_pool).await?;

    let messages = get_messages_from_chat_id(chat.id, &db_pool).await?;

    let chat = Chat {
        id,
        name: chat.name.unwrap_or_default(), // default is an empty string
        description: chat.description,
        messages,
        is_from_perspective_a: true,
    };

    Ok(chat)
}

pub(crate) async fn get_chat_by_id(
    /*     user: Claims,
     */ State(state): State<crate::State>,
    Path(chat_id): Path<Uuid>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let chat = load_chat_from_db(chat_id, &state.db_pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::OK, json!(chat).to_string()))
}

pub(crate) async fn get_chat_by_id_from_user_perspective(
    user: Claims,
    State(state): State<crate::State>,
    Path(chat_id): Path<Uuid>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    // I used a closure here because it allowed me to map all sql errors at once later
    let chat: Result<Chat, sqlx::Error> = (async || {
        let chat_row = get_chat_row_from_id(chat_id, &state.db_pool).await?;

        // TODO: theoretically, these 2 request could run concurrently
        let mut messages = get_messages_from_chat_id(chat_id, &state.db_pool).await?;

        // if the user is user b, invert the message perspectives
        let invert_chat = Some(user.id) == chat_row.user_id_b && user.id != chat_row.user_id_a;
        if invert_chat {
            messages.iter_mut().for_each(|msg| {
                msg.is_own = !msg.is_own;
            });
        }
        Ok(Chat {
            id: chat_row.id,
            description: chat_row.description,
            name: chat_row.name.unwrap_or_default(),
            messages,
            is_from_perspective_a: !invert_chat,
        })
    })()
    .await;

    match chat {
        Ok(chat) => Ok((StatusCode::OK, json!(chat).to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

#[derive(Deserialize)]
pub(crate) struct NewChat {
    pub(crate) name: String,
    pub(crate) description: Option<String>,
}

pub(crate) async fn create_chat(
    user: Claims,
    State(state): State<crate::State>,
    Json(chat): Json<NewChat>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let chat_id = Uuid::new_v4();

    // This creates a recreated chat (because both user ids are the same)
    if sqlx::query!(
        "INSERT INTO chats (id, name, description, user_id_a, user_id_b) VALUES ($1,$2,$3,$4,$4)",
        chat_id,
        chat.name,
        chat.description,
        user.id
    )
    .execute(&state.db_pool)
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

pub(crate) async fn create_chat_with_random(
    user: Claims,
    State(state): State<crate::State>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    // TODO: Make sure a user can only create one random request

    // Check if there are open chat requests that can be accepted
    let chat = sqlx::query_as!(
        ChatRow,
        "SELECT * FROM chats WHERE user_id_b IS NULL AND user_id_a != $1",
        user.id
    )
    .fetch_optional(&state.db_pool)
    .await;

    let chat = match chat {
        Ok(val) => val,
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }
    };

    if let Some(chat) = chat {
        // Close the chat request
        let res = sqlx::query!(
            "UPDATE chats SET user_id_b = $1 WHERE id = $2",
            user.id,
            chat.id
        )
        .execute(&state.db_pool)
        .await;

        match res {
            Ok(_) => Ok((StatusCode::OK, format!("{}", chat.id))),
            Err(e) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to accept open request: {}", e),
            )),
        }
    } else {
        // No chat request exists, create a new one
        let chat_id = Uuid::new_v4();
        let res = sqlx::query!("INSERT INTO chats (id, name, description, user_id_a, user_id_b) VALUES ($1,$2,NULL,$3, NULL)",
            chat_id,
            String::from("New random chat"),
            user.id
        ).execute(&state.db_pool).await;

        match res {
            Ok(_) => Ok((StatusCode::CREATED, String::from("Created chat request"))),
            Err(e) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create new chat request: {}", e),
            )),
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct NewMessage {
    #[serde(alias = "chatId")]
    pub(crate) chat_id: Uuid,
    pub(crate) content: String,
    #[serde(alias = "isOwn")]
    pub(crate) is_owned_by_a: bool,
    pub(crate) index: i32,
}

pub(crate) async fn post_message(
    user: Claims,
    State(state): State<crate::State>,
    Json(mut message): Json<NewMessage>,
) -> Result<StatusCode, (StatusCode, String)> {
    // Check if the specified chat belongs to the user
    let user_ids = sqlx::query!(
        "SELECT user_id_a AS a, user_id_b AS b FROM chats WHERE id=$1 AND user_id_b IS NOT NULL AND (user_id_a=$2 OR user_id_b=$2) LIMIT 1",
        message.chat_id,
        user.id
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            String::from("The chat doesn't exist or doesn't belong to you"),
        )
    })?;

    let is_recreated_chat = user_ids.a == user_ids.b.unwrap(); // The sql query ensures that user_id_b is never null

    // If this is a chat with another person, ignore the is_owned property
    // And always send the message from the perspective of the poster
    if !is_recreated_chat {
        message.is_owned_by_a = user.id == user_ids.a;
    }

    /*
       // Now, check if the user is allowed to send the message for that side
       if !is_recreated_chat && (message.is_owned_by_a ^ (user.id == user_ids.a)) {
           return Err((
               StatusCode::BAD_REQUEST,
               "User is not allowed to post a message from that side".to_owned(),
           ));
       }
    */

    // At this point, the query would've thrown an error if the chat doesn't exist or doesn't belong to the user

    let message_id = Uuid::new_v4();

    let query = sqlx::query!(
        "INSERT INTO chat_messages (id, content, chat_id, index, is_owned_by_a) VALUES ($1,$2,$3,$4,$5)",
        message_id,
        message.content,
        message.chat_id,
        message.index,
        message.is_owned_by_a
    );

    if let Ok(_) = query.execute(&state.db_pool).await {
        let msg = Message {
            id: message_id,
            content: message.content,
            is_own: message.is_owned_by_a,
            avg_rating: 0f64,
        };

        state
            .synchronizer
            .lock()
            .await
            .post_message(message.chat_id, msg);
        Ok(StatusCode::CREATED)
    } else {
        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("creating message failed"),
        ))
    }
}

#[derive(Deserialize)]
pub struct NewComment {
    #[serde(alias = "messageId")]
    pub message_id: Uuid,
    pub content: String,
}

pub(crate) async fn post_comment(
    user: Claims,
    State(state): State<crate::State>,
    Json(comment): Json<NewComment>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let id = Uuid::new_v4();

    let res = sqlx::query!(
        "INSERT INTO comments (id, message_id, owner_id, content, time) VALUES ($1,$2,$3,$4,$5)",
        id,
        comment.message_id,
        user.id,
        comment.content,
        chrono::Utc::now()
    )
    .execute(&state.db_pool)
    .await;

    if res.is_ok() {
        Ok((StatusCode::CREATED, id.to_string()))
    } else {
        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            res.unwrap_err().to_string(),
        ))
    }
}

struct CommentRow {
    id: Uuid,
    message_id: Uuid,
    owner_id: Uuid,
    content: String,
    time: Option<DateTime<Utc>>,
    owner_name: String,
}

#[derive(Serialize)]
struct Comment {
    id: Uuid,
    #[serde(rename = "ownerId")]
    owner_id: Uuid,
    content: String,
    #[serde(rename = "ownerName")]
    owner_name: String,
    timestamp: i64,
}

pub(crate) async fn get_comments_for_message(
    State(state): State<crate::State>,
    Path(message_id): Path<Uuid>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let result = sqlx::query_as!(
        CommentRow,
        r#"
SELECT comments.*, users.username as owner_name FROM comments 
JOIN users ON users.id = comments.owner_id
WHERE message_id = $1
        "#,
        message_id
    )
    .fetch_all(&state.db_pool)
    .await;

    match result {
        Ok(comments) => Ok((
            StatusCode::OK,
            json!(
                comments
                    .into_iter()
                    .map(|row| {
                        Comment {
                            id: row.id,
                            owner_id: row.owner_id,
                            content: row.content,
                            owner_name: row.owner_name,
                            timestamp: row.time.unwrap().timestamp(),
                        }
                    })
                    .collect::<Vec<Comment>>()
            )
            .to_string(),
        )),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

#[derive(Deserialize)]
struct WebSocketHandshake {
    token: Option<String>,
}

pub(crate) async fn websocket_chat_updates(
    ws: WebSocketUpgrade,
    State(state): State<crate::State>,
    Path(chat_id): Path<Uuid>,
) -> Response<Body> {
    let mut synchronizer = state.synchronizer.lock().await;
    let mut receiver = synchronizer.get_receiver(chat_id);
    drop(synchronizer);

    let Ok(chat) = get_chat_row_from_id(chat_id, &state.db_pool).await else{
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::new("The chat does not exist (or the db failed lol)".to_owned()))
            .unwrap();
    };

    ws.on_upgrade(async move |mut socket| {
        let mut user: Option<Claims> = None;
        if let Ok(Some(Ok(msg))) = tokio::time::timeout(Duration::from_millis(3), socket.recv()).await
        {
            let Ok(Ok(data)) = msg
                .to_text()
                .and_then(|txt| Ok(serde_json::from_str::<WebSocketHandshake>(txt)))
            else {
                return;
            };

            let claims = data.token.and_then(|token|{ Some(auth::validate_token(&token)) });

            if let Some(Ok(claims)) = claims{
                user = Some(claims);
            }
        }

        let invert_messages = if let Some(user) = user{
            chat.user_id_a != user.id && chat.user_id_b == Some(user.id)
        }else{ false };

        // Ignore the current value, we're only interesting in changes
        receiver.mark_unchanged();
        loop {
            if receiver.changed().await.is_err() {
                let _ = socket
                    .send(ws::Message::Close(Some(CloseFrame {
                        code: 500u16,
                        reason: "Synchronization error".into(),
                    })))
                    .await;
                println!("Sync error!");
                break;
            }

            // Cloning the value somehow fixes the fact that the
            // smart pointer coming out of borrow() is not `Send`
            // Axum enforces that the future coming out of this async closure is `Send` though
            let update = receiver.borrow().clone();

            if let Update::MessageAdded(mut msg) = update {

                msg.is_own ^= invert_messages;

                let json_msg = json!(msg).to_string() + "\n";
                let res = socket.send(ws::Message::text(json_msg)).await;

                if res.is_err() {
                    break;
                }
            }
        }
    })
}
