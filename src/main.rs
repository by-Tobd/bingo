use app_state::AppData;
use envconfig::Envconfig;
use actix_web::{App, HttpServer, Responder, get, web};

mod config;
mod routes;
mod app_state;
mod model;

use config::Config;
use model::database::Database;

use actix_web::{App, HttpServer, Responder, get, web};
use envconfig::Envconfig;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::init_from_env().unwrap();
    let db = Database::new(&config).await.unwrap();

    env_logger::init();
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
