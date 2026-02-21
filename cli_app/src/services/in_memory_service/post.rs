use std::{collections::HashMap, sync::Mutex};

use anyhow::Ok;

use crate::{
    model::Post,
    services::post::{CreatePostRequest, PostService, UpdatePostRequest},
};

pub struct InMemoryPostStore {
    pub counter: i64,
    pub items: HashMap<i64, Post>,
}

pub struct InMemoryPostService {
    data: Mutex<InMemoryPostStore>,
}

impl Default for InMemoryPostService {
    fn default() -> Self {
        Self {
            data: Mutex::new(InMemoryPostStore {
                counter: 0,
                items: Default::default(),
            }),
        }
    }
}

impl PostService for InMemoryPostService {
    async fn get_all_posts(&self) -> anyhow::Result<Vec<Post>> {
        let data = self.data.lock().unwrap();
        Ok(data.items.values().map(|post| (*post).clone()).collect())
    }

    async fn get_post_by_id(&self, id: i64) -> anyhow::Result<Post> {
        let data = self.data.lock().unwrap();

        match data.items.get(&id) {
            Some(post) => Ok((*post).clone()),
            None => anyhow::bail!("Post not found: {}", id),
        }
    }

    async fn get_post_by_slug(&self, name: &str) -> anyhow::Result<Post> {
        let data = self.data.lock().unwrap();
        for (_id, post) in data.items.iter() {
            if post.slug == name {
                return Ok(post.clone());
            }
        }

        anyhow::bail!("Post not found: {}", name)
    }

    async fn delete_post(&self, id: i64) -> anyhow::Result<()> {
        let mut data = self.data.lock().unwrap();
        match data.items.remove(&id) {
            None => {
                anyhow::bail!("Post not found: {}", id)
            }
            Some(_) => Ok(()),
        }
    }

    async fn update_post(&self, id: &i64, req: UpdatePostRequest) -> anyhow::Result<Post> {
        let mut data = self.data.lock().unwrap();
        let post = data
            .items
            .get_mut(id)
            .ok_or(anyhow::anyhow!("Post not found: {}", req.id))?;

        post.slug = req.slug;
        post.title = req.title;
        post.content = req.content;
        post.status = req.status;

        match data.items.get(id) {
            None => {
                anyhow::bail!("Post not found: {}", req.id)
            }
            Some(post) => Ok(post.clone()),
        }
    }
    async fn create_post(&self, req: CreatePostRequest) -> anyhow::Result<Post> {
        let mut data = self.data.lock().unwrap();
        data.counter += 1;
        let ts = chrono::offset::Utc::now();
        let post = Post {
            id: data.counter,
            author_id: req.author_id,
            slug: req.slug,
            title: req.title,
            content: req.content,
            status: req.status,
            created: ts,
            updated: ts,
        };

        data.items.insert(post.id, post);

        match data.items.get(&data.counter) {
            None => {
                anyhow::bail!("Post not found: {}", data.counter)
            }
            Some(post) => Ok(post.clone()),
        }
    }
}
