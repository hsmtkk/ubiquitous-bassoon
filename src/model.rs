use serde::{Deserialize, Serialize};

pub struct Post {
    id: i64,
    content: String,
    author: String,
}
