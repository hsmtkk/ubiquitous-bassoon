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
        let conn = pool.get().unwrap();
        let js = serde_json::to_string(&Post::new(0, "Example", "Alice")).unwrap();
        conn.set("0", "hogehoge");
        PostRepository { pool }
    }

    pub fn retrieve(&self, id: i64) -> Option<Post> {
        let conn = self.pool.get().unwrap();
        let resp: Option<String> = conn.get(format!("{}", id)).unwrap();
        match resp {
            Some(resp) => {
                let post: Post = serde_json::from_str(&resp).unwrap();
                Some(post)                    
            },
            None => None,
        }
    }

    pub fn create(&mut self, post:Post) -> Result<()> {
        unimplemented!()
    }

    pub fn update(&mut self, post:Post) -> Result<()> {
        unimplemented!()
    }

    pub fn delete(&mut self, post:Post) -> Result<()> {
        unimplemented!()
    }
}
