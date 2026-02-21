use crate::{
    services::in_memory_service::{post::InMemoryPostService, user::InMemoryUserService},
    settings::Settings,
};
use anyhow::Ok;
use arc_swap::ArcSwap;
use std::sync::Arc;

pub struct ApplicationState {
    pub settings: ArcSwap<Settings>,
    pub post_service: Arc<InMemoryPostService>,
    pub user_service: Arc<InMemoryUserService>,
}

impl ApplicationState {
    pub fn new(settings: &Settings) -> anyhow::Result<Self> {
        Ok(Self {
            settings: ArcSwap::new(Arc::new((*settings).clone())),
            post_service: Arc::new(InMemoryPostService::default()),
            user_service: Arc::new(InMemoryUserService::default()),
        })
    }
}
