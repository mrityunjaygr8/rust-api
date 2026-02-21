use std::{collections::HashMap, sync::Mutex};

use crate::{
    model::{AppError, User},
    services::user::{LoginUserRequest, RegisterUserRequest, UpdateUserRequest, UserService},
};

pub struct InMemoryUserStore {
    pub counter: i64,
    pub items: HashMap<String, User>,
}

pub struct InMemoryUserService {
    data: Mutex<InMemoryUserStore>,
}

impl Default for InMemoryUserService {
    fn default() -> Self {
        Self {
            data: Mutex::new(InMemoryUserStore {
                counter: 0,
                items: Default::default(),
            }),
        }
    }
}

impl UserService for InMemoryUserService {
    async fn login(&self, req: LoginUserRequest) -> anyhow::Result<User, AppError> {
        let data = self.data.lock().unwrap();

        match data.items.get(&req.username) {
            Some(user) => {
                if user.username == req.password {
                    return Ok(user.clone());
                } else {
                    return Err(AppError::BadRequest {
                        message: "Username or password does not match".to_string(),
                    });
                }
            }
            None => {
                return Err(AppError::BadRequest {
                    message: "Username or password does not match".to_string(),
                });
            }
        }
    }

    async fn update(&self, id: String, req: UpdateUserRequest) -> anyhow::Result<User, AppError> {
        let mut data = self.data.lock().unwrap();

        let user = data.items.get_mut(&id.clone()).ok_or(AppError::NotFound {
            id: id.clone(),
            item_type: crate::model::Models::UserModel,
        })?;

        user.last_login = req.last_login;
        match data.items.get(&id) {
            None => {
                return Err(AppError::NotFound {
                    id,
                    item_type: crate::model::Models::UserModel,
                });
            }
            Some(user) => Ok(user.clone()),
        }
    }

    async fn register(&self, req: RegisterUserRequest) -> anyhow::Result<(), AppError> {
        let mut data = self.data.lock().unwrap();
        data.counter += 1;

        let ts = chrono::offset::Utc::now();
        let key = format!("{}", req.username.clone());

        let user = User {
            id: data.counter,
            username: req.username,
            status: crate::model::UserStatus::Active,
            password: req.password,
            created: ts,
            updated: ts,
            last_login: None,
        };

        data.items.insert(key, user);

        Ok(())
    }
}
