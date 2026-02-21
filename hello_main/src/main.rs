use anyhow::{Context, anyhow};
use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};
use rand::RngExt;
use serde::Serialize;
use serde_json::json;
use thiserror::Error;

#[derive(Serialize)]
struct Response {
    message: String,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("resource not found")]
    NotFound,

    #[error("authentication failed")]
    Unauthorized,

    #[error("internal server error: {0}")]
    InternalServerError(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        use AppError::*;
        let (status, message) = match self {
            NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            InternalServerError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };
        (status, Json(json!({"error": message}))).into_response()
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(hello_json))
        .layer(tower_http::catch_panic::CatchPanicLayer::new());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .context("failed to bind TCP listener")?;
    axum::serve(listener, app)
        .await
        .context("axum::serve failed")?;
    Ok(())
}

#[axum::debug_handler]
async fn hello_json() -> Result<(StatusCode, Json<Response>), AppError> {
    let response = Response {
        message: generate_message()?,
    };
    Ok((StatusCode::OK, Json(response)))
}

fn generate_message() -> Result<String, AppError> {
    let mut rng = rand::rng();
    match rng.random_range(1..=4) {
        1 => Ok("hello".to_string()),
        2 => Err(AppError::Unauthorized),
        3 => Err(AppError::NotFound),
        4 => Err(AppError::InternalServerError(anyhow!("whoo"))),
        _ => todo!(),
    }
}
