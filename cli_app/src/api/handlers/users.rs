use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{
    model::AppError,
    services::user::{RegisterUserRequest, UserService},
    state::ApplicationState,
};

pub async fn register(
    State(state): State<Arc<ApplicationState>>,
    Json(payload): Json<RegisterUserRequest>,
) -> Result<(), AppError> {
    state.user_service.register(payload).await
}
