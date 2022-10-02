use actix_web::{HttpServer, App};
use controllers::novel_controller;

mod domain;
mod core;
mod data;
mod controllers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(novel_controller::get_novel_list)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
