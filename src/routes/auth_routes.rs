use actix_web::web;

use super::handlers::auth_handlers;


pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api")
        .service(auth_handlers::register)
    );
}