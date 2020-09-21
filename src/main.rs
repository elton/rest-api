use actix_web::{get, middleware, App, HttpResponse, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod json;

#[get("/hi")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    println!("Started http server: 127.0.0.1:8443");
    // 使用mkcert创建本地证书（https://github.com/FiloSottile/mkcert）
    // brew install mkcert
    // mkcert localhost 127.0.0.1 ::1
    // load ssl keys
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();
    json::run();
    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(index)
    })
    .bind_openssl("127.0.0.1:8443", builder)?
    .run()
    .await
}
