use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Post {
    pub id: i64,
    pub content: String,
    pub author: String,
}

impl Post {
    pub fn new(id:i64, content:&str, author:&str) -> Post {
        Post{id, content:content.to_string(), author:author.to_string()}
    }
}