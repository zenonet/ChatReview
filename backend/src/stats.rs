use axum::{extract::State, http::StatusCode};
use serde::Serialize;
use serde_json::json;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct Stat {
    name: Option<String>,
    value: Option<f64>,
}

pub(crate) async fn get_stats(
    State(state): State<crate::State>,
) -> Result<String, (StatusCode, String)> {
    // TODO: Implement caching stats

    let res = sqlx::query_as!(
        Stat,
        r#"
SELECT 'Total message comments' AS name, count(1) AS value FROM comments
UNION SELECT 'Total ratings', count(1) FROM message_ratings
UNION SELECT 'Total messages', count(1) FROM chat_messages
UNION SELECT 'Total chats', count(1) FROM chats
UNION SELECT 'Average message rating', avg(value) FROM message_ratings
"#
    )
    .fetch_all(&state.db_pool)
    .await;

    match res {
        Ok(stats) => {
            Ok(json!(stats).to_string())
        }
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}
