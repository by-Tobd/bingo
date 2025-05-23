use app_state::AppData;
use envconfig::Envconfig;
use actix_web::{App, HttpServer, Responder, get, web};

mod config;
mod routes;
mod app_state;
mod model;
mod util;

use config::Config;
use log::warn;
use model::database::Database;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::init_from_env().unwrap();
    let db = Database::new(&config).await.unwrap();

    env_logger::init();
    if config.jwt_secret == "insecure" {
        warn!("Using insecure jwt secret!");
    }

    let data = AppData {
        db,
        _config: config.clone()
    };
    HttpServer::new(move || 
            App::new()
            .app_data(web::Data::new(data.clone()))
            .configure(routes::auth_routes::config)
            .service(greet)
        )
        .bind((config.bind_addr, config.port))?
        .run()
        .await
}
