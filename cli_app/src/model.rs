use axum::{Json, http::StatusCode, response::IntoResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum UserStatus {
    Active = 1,
    Blocked = 2,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum PostStatus {
    Draft = 1,
    Published = 2,
}

#[derive(Clone, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub status: UserStatus,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
}

#[derive(Clone, Serialize)]
pub struct Post {
    pub id: i64,
    pub author_id: i64,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub status: PostStatus,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not found")]
    NotFound,

    #[error(transparent)]
    InternalServerError(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        use AppError::*;
        let (status_code, message) = match self {
            NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            InternalServerError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };
        (status_code, Json(json!({"error": message}))).into_response()
    }
}
