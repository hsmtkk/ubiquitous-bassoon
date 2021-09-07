use crate::model::Post;
use anyhow::Result;
use std::collections::HashMap;

#[derive(Clone)]
pub struct PostRepository {
    memory_cache: HashMap<i64, Post>,
}

impl PostRepository {
    pub fn new() -> PostRepository {
        let memory_cache: HashMap<i64, Post> = HashMap::new();
        PostRepository { memory_cache }
    }

    pub fn retrieve(&self, id: i64) -> Option<Post> {
        match self.memory_cache.get(&id) {
            Some(ref_post) => Some(Post::new(ref_post.id, &ref_post.content, &ref_post.author)),
            None => None,
        }
    }

    pub fn create(&mut self, post:Post) -> Result<()> {
        self.memory_cache.insert(post.id, post);
        Ok(())
    }

    pub fn update(&mut self, post:Post) -> Result<()> {
        self.memory_cache.insert(post.id, post);
        Ok(())
    }

    pub fn delete(&mut self, post:Post) -> Result<()> {
        self.memory_cache.remove(&post.id);
        Ok(())
    }
}
