// Copyright 2020 Elton Zheng
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use crate::config::CONFIG;
use crate::routes::routes;

use actix_web::{middleware::Logger, App, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

/// Setup a Server using Actix-web 3
pub async fn server() -> std::io::Result<()> {
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
    let server = HttpServer::new(move || {
        App::new()
            // enable logger
            .wrap(Logger::default())
            .configure(routes)
    })
    .bind_openssl(&CONFIG.server, builder)?;
    server.run().await
}
