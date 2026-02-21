use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::model::{AppError, User};

#[allow(async_fn_in_trait)]
pub trait UserService {
    async fn register(&self, req: RegisterUserRequest) -> anyhow::Result<(), AppError>;
    async fn login(&self, req: LoginUserRequest) -> anyhow::Result<User, AppError>;
    async fn update(&self, id: String, req: UpdateUserRequest) -> anyhow::Result<User, AppError>;
}

#[derive(Clone, Deserialize)]
pub struct RegisterUserRequest {
    pub username: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Clone, Deserialize)]
pub struct UpdateUserRequest {
    pub last_login: Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize)]
pub struct LoginUserRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
}
