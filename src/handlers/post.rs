use crate::database::MysqlPool;
use crate::handlers::mysql_pool_handler;
use crate::models::post::{NewPost, PostList};
use actix_web::{get, web, HttpRequest, HttpResponse};
use serde_json::Result;

#[get("/posts")]
pub async fn get_posts(_req: HttpRequest, pool: web::Data<MysqlPool>) -> Result<HttpResponse> {
    let connection = mysql_pool_handler(pool).unwrap();
    Ok(HttpResponse::Ok().json(PostList::list(&connection)))
}

// pub async fn create_post() {}
