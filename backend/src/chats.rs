
use axum::extract::Path;
use axum::Json;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use axum::http::StatusCode;
use sqlx::{FromRow, PgPool};
use axum::extract::State;
use crate::auth::Claims;


#[derive(Serialize, Deserialize, FromRow)]
struct ChatRow {
    id: Uuid,
    name: Option<String>,
    description: Option<String>,
    owner_id: Uuid,
}

#[derive(Serialize)]
pub struct Chat {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
struct MessageRow {
    id: Uuid,
    content: Option<String>,

    #[serde(rename = "isOwn")]
    is_own: Option<bool>,

    #[serde(rename = "chatId")]
    chat_id: Option<Uuid>,
    index: i32,

    avg_rating: Option<f64>
}

#[derive(Serialize)]
pub struct Message {
    #[serde(default)]
    pub id: Uuid,
    pub content: String,
    #[serde(rename = "isOwn")]
    pub is_own: bool,

    pub avg_rating: f64
}


pub(crate) async fn get_my_chats(
    user: Claims,
    State(db_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let rows = sqlx::query_as!(ChatRow, "SELECT * FROM chats WHERE owner_id=$1", user.id)
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

#[derive(Deserialize)]
pub(crate) struct NewRating {
    pub(crate) value: f64,
    #[serde(alias = "messageId")]
    pub(crate) message_id: Uuid,
}

pub(crate) async fn post_rating(
    user: Claims,
    State(db_pool): State<PgPool>,
    Json(rating): Json<NewRating>,
) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query!("INSERT INTO message_ratings (message_id, owner_id, value, changed) VALUES ($1,$2,$3,$4) ON CONFLICT (message_id,owner_id) DO UPDATE SET value = $3, changed = $4",
            rating.message_id,
            user.id,
            rating.value,
            chrono::Utc::now()
        ).execute(&db_pool).await.map_err(|e|{
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
    m.is_own,
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
            is_own: row.is_own.unwrap_or_else(|| true),
            avg_rating: row.avg_rating.unwrap_or(0f64),
        })
        .collect::<Vec<Message>>())
}

// TODO: Investigate how to return objects directly and let them be serialized by axum
pub(crate) async fn get_random_chat(
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
    let messages = get_messages_from_chat_id(chat_row.id, &db_pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let chat = Chat {
        id: chat_row.id,
        name: chat_row.name.unwrap(),
        description: chat_row.description,
        messages,
    };

    Ok((StatusCode::OK, json!(chat).to_string()))
}

pub(crate) async fn get_chat_by_id(
    user: Claims,
    State(db_pool): State<PgPool>,
    Path(chat_id): Path<Uuid>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let chat = sqlx::query_as!(
        ChatRow,
        "SELECT * FROM chats WHERE id=$1 AND owner_id=$2 LIMIT 1",
        chat_id,
        user.id
    )
    .fetch_optional(&db_pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let Some(chat) = chat else {
        return Err((
            StatusCode::BAD_REQUEST,
            String::from("A chat with the specified id could not be found"),
        ));
    };

    let messages = get_messages_from_chat_id(chat.id, &db_pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((
        StatusCode::OK,
        json!(Chat {
            id: chat_id,
            name: chat.name.unwrap(),
            description: chat.description,
            messages
        })
        .to_string(),
    ))
}

#[derive(Deserialize)]
pub(crate) struct NewChat {
    pub(crate) name: String,
    pub(crate) description: Option<String>,
}

pub(crate) async fn create_chat(
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
pub(crate) struct NewMessage {
    #[serde(alias = "chatId")]
    pub(crate) chat_id: Uuid,
    pub(crate) content: String,
    #[serde(alias = "isOwn")]
    pub(crate) is_own: bool,
    pub(crate) index: i32,
}

pub(crate) async fn post_message(
    user: Claims,
    State(db_pool): State<PgPool>,
    Json(message): Json<NewMessage>,
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


#[derive(Deserialize)]
pub struct NewComment{
    #[serde(alias = "messageId")]
    pub message_id: Uuid,
    pub content: String,
}

pub(crate) async fn post_comment(
    user: Claims,
    State(db_pool): State<PgPool>,
    Json(comment): Json<NewComment>
) -> Result<(StatusCode, String), (StatusCode, String)>{

    let id = Uuid::new_v4(); 

    let res = sqlx::query!("INSERT INTO comments (id, message_id, owner_id, content, time) VALUES ($1,$2,$3,$4,$5)",
        id,
        comment.message_id,
        user.id,
        comment.content,
        chrono::Utc::now()
    ).execute(&db_pool).await;

    if res.is_ok(){
        Ok((
            StatusCode::CREATED,
            id.to_string()
        ))
    }else{
        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            res.unwrap_err().to_string()
        ))
    }
}


struct CommentRow{
    id: Uuid,
    message_id: Uuid,
    owner_id: Uuid,
    content: String,
    time: Option<DateTime<Utc>>
}

#[derive(Serialize)]
struct Comment{
    id: Uuid,
    #[serde(alias = "ownerId")]
    owner_id: Uuid,
    content: String
}

pub(crate) async fn get_comments_for_message(
    State(db_pool): State<PgPool>,
    Path(message_id): Path<Uuid>
) -> Result<(StatusCode, String), (StatusCode, String)>{

    let result = sqlx::query_as!(CommentRow, "SELECT * FROM comments WHERE message_id = $1",
        message_id
    ).fetch_all(&db_pool).await;


    match result {
        Ok(comments) => {
            Ok((
                StatusCode::OK,
                json!(comments.into_iter().map(|row|{
                    Comment{
                        id: row.id,
                        owner_id: row.owner_id,
                        content: row.content
                    }
                }).collect::<Vec<Comment>>()).to_string()
            ))
        },
        Err(e) => {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                e.to_string()
            ))
        }
    }
}