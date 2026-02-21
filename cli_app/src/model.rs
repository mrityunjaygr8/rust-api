use std::fmt::Display;

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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Models {
    UserModel = 1,
    PostModel = 2,
}

impl Display for Models {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Models::*;
        match self {
            UserModel => write!(f, "User"),
            PostModel => write!(f, "Post"),
        }
    }
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
    #[error("{item_type} Not found: {id}")]
    NotFound { id: String, item_type: Models },

    #[error("{message}")]
    BadRequest { message: String },

    #[error(transparent)]
    InternalServerError(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        use AppError::*;
        let (status_code, message) = match self {
            NotFound {
                id: _,
                item_type: _,
            } => (StatusCode::NOT_FOUND, self.to_string()),
            BadRequest { message: _ } => (StatusCode::BAD_REQUEST, self.to_string()),
            InternalServerError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };
        (status_code, Json(json!({"error": message}))).into_response()
    }
}
