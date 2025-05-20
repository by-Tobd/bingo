use actix_web::{post, web::{self, Data}, HttpResponse, Responder};
use argon2::{password_hash::{rand_core::OsRng, SaltString}, Argon2, PasswordHasher};
use log::error;
use sea_orm::{ActiveModelTrait, ActiveValue::Set};
use serde::Deserialize;

use crate::{app_state::AppData, model::user};

#[derive(Deserialize)]
struct RegisterModel {
    name: String,
    password: String
}

#[post("register")]
pub async fn register(
    app_state: Data<AppData>,
    register_json: web::Json<RegisterModel>
) -> impl Responder {
    if register_json.password.len() < app_state._config.min_password_length {
        return HttpResponse::BadRequest().body(format!("Password must be at least {} characters long", app_state._config.min_password_length));
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon = Argon2::default();
    let hash = argon.hash_password(register_json.password.as_bytes(), &salt);
    
    if let Err(err) = hash {
        error!("Couldn't hash password: {}", err);
        return HttpResponse::InternalServerError().body(err.to_string());
    } 

    let res = user::ActiveModel {
        name: Set(register_json.name.clone()),
        hashed_password: Set(hash.unwrap().to_string()),
        salt: Set(salt.to_string()),
        ..Default::default()
    }.save(&app_state.db.connection).await;

    if let Err(err) = res {
        return match err {
            sea_orm::DbErr::Exec(err) => HttpResponse::BadRequest().body(err.to_string()),
            err => return HttpResponse::InternalServerError().body(err.to_string()),
        }
    }

    HttpResponse::Ok().body("Successfully registered")
}