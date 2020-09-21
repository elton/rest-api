#[macro_use]
extern crate lazy_static;

use crate::server::server;

mod config;
mod handlers;
mod routes;
mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server().await
}
