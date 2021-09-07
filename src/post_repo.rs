use crate::model::Post;
use r2d2::Pool;
use redis::{Commands, Client};

#[derive(Clone, Debug)]
pub struct PostRepository {
    pool: Pool<Client>,
}

impl PostRepository {
    pub fn new(pool:Pool<Client>) -> PostRepository {
        let mut conn = pool.get().unwrap();
        let sample_post = Post::new(0, "Example", "Alice");
        let js = serde_json::to_string(&sample_post).unwrap();
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

    pub fn create(&self, post:Post) {
        let mut conn = self.pool.get().unwrap();
        let js = serde_json::to_string(&post).unwrap();
        let _:() = conn.set(post.id, js).unwrap();
    }

    pub fn update(&self, post:Post)  {
        let mut conn = self.pool.get().unwrap();
        let js = serde_json::to_string(&post).unwrap();
        let _:() = conn.set(post.id, js).unwrap();
    }

    pub fn delete(&self, id:i64)  {
        let mut conn = self.pool.get().unwrap();
        let _: () = conn.del(id).unwrap();
    }
}
