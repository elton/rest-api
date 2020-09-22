use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct postResponse {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct postsResponse(pub Vec<postResponse>);
