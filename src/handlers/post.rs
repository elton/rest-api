use crate::models::post::{get_all, NewPost};
use actix_web::{get, web::block};
use serde_json::Result;

#[get("/posts")]
pub async fn get_posts() -> Result<String> {
    let posts = block(move || get_all()).await;
    serde_json::to_string(&posts.unwrap())
}

pub async fn create_post() {}
