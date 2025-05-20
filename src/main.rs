mod config;

use config::Config;

use actix_web::{App, HttpServer, Responder, get, web};
use envconfig::Envconfig;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::init_from_env().unwrap();

    HttpServer::new(|| App::new().service(greet))
        .bind((config.bind_addr, config.port))?
        .run()
        .await
}
