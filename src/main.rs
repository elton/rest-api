#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate diesel;

use crate::server::server;

mod config;
mod database;
mod handlers;
mod models;
mod routes;
mod schema;
mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server().await
}
