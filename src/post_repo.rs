use crate::model::Post;
use anyhow::Result;
use r2d2::Pool;
use redis::{Commands, Client};

#[derive(Clone)]
pub struct PostRepository {
    pool: Pool<Client>,
}

impl PostRepository {
    pub fn new(pool:Pool<Client>) -> PostRepository {
        let mut conn = pool.get().unwrap();
        let js = serde_json::to_string(&Post::new(0, "Example", "Alice")).unwrap();
        let _:() = conn.set(0, js).unwrap();
        PostRepository { pool }
    }

    pub fn retrieve(&self, id: i64) -> Option<Post> {
        let mut conn = self.pool.get().unwrap();
        let resp: String = conn.get(id).unwrap_or_else(|_| "".to_string());
        if resp.is_empty() {
            return None;
        }
        let post: Post = serde_json::from_str(&resp).unwrap();
        Some(post)
    }

    pub fn create(&mut self, post:Post) -> Result<()> {
        let mut conn = self.pool.get().unwrap();
        let js = serde_json::to_string(&post).unwrap();
        let _:() = conn.set(post.id, js).unwrap();
        Ok(())
    }

    pub fn update(&mut self, post:Post) -> Result<()> {
        let mut conn = self.pool.get().unwrap();
        let js = serde_json::to_string(&post).unwrap();
        let _:() = conn.set(post.id, js).unwrap();
        Ok(())
    }

    pub fn delete(&mut self, id:i64) -> Result<()> {
        let mut conn = self.pool.get().unwrap();
        let _: () = conn.del(id).unwrap();
        Ok(())
    }
}
