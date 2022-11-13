use axum::{response::IntoResponse, Extension, Form};
use serde::Deserialize;
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

const INSERT_MESSAGE_SQL: &str = "INSERT INTO messages (message_sid, account_id, content, from_number, to_number) VALUES (?, ?, ?, ?, ?)";

#[allow(dead_code, non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct TwilioMessage {
    pub AccountSid: String,
    pub MessageSid: String,
    pub Body: String,
    pub From: String,
    pub To: String,
}

pub async fn receive_message(
    payload: Form<TwilioMessage>,
    Extension(db): Extension<Arc<Pool<Sqlite>>>,
) -> impl IntoResponse {
    println!("Received message: {:#?}", payload);

    match sqlx::query(INSERT_MESSAGE_SQL)
        .bind(&payload.MessageSid)
        .bind(&payload.AccountSid)
        .bind(&payload.Body)
        .bind(&payload.From)
        .bind(&payload.To)
        .execute(&*db)
        .await
    {
        Ok(_) => axum::http::StatusCode::NO_CONTENT,
        Err(e) => {
            println!("Error inserting message: {}", e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
