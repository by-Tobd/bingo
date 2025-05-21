use actix_web::{post, web::{self, Data}, HttpResponse, Responder};
use argon2::{password_hash::{rand_core::OsRng, SaltString}, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use log::error;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, EntityTrait, QueryFilter};
use serde::Deserialize;

use crate::{app_state::AppData, model::user, util::jwt::encode_jwt};

#[derive(Deserialize)]
struct UserModel {
    name: String,
    password: String
}

#[post("register")]
pub async fn register(
    app_state: Data<AppData>,
    register_json: web::Json<UserModel>
) -> impl Responder {
    if register_json.password.len() < app_state._config.min_password_length {
        return HttpResponse::BadRequest().body(format!("Password must be at least {} characters long", app_state._config.min_password_length));
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon = Argon2::default();
    let hash = argon.hash_password(register_json.password.as_bytes(), &salt);
    
    if let Err(err) = hash {
        error!("Couldn't hash password: {}", err);
        return HttpResponse::InternalServerError().body(err.to_string()); //FIXME
    } 

    let res = user::ActiveModel {
        name: Set(register_json.name.clone()),
        hashed_password: Set(hash.unwrap().to_string()),
        salt: Set(salt.to_string()),
        ..Default::default()
    }.save(&app_state.db.connection).await;

    if let Err(err) = res {
        return match err {
            sea_orm::DbErr::Exec(err) => HttpResponse::BadRequest().body(err.to_string()), //FIXME
            err => return HttpResponse::InternalServerError().body(err.to_string()), //FIXME
        }
    }

    HttpResponse::Created().body("Successfully registered")
}

#[post("/login")]
pub async fn login(
    app_state: web::Data<AppData>,
    login_json: web::Json<UserModel>
) -> impl Responder {
    let user = user::Entity::find().filter(
        Condition::all()
        .add(user::Column::Name.eq(&login_json.name))
    ).one(&app_state.db.connection).await;
    match user {
        Ok(Some(user)) => {
            let argon = Argon2::default();

            match PasswordHash::new(&user.hashed_password) {
                Ok(hash) => {
                    if let Err(_) = argon.verify_password(login_json.password.as_bytes(), &hash) {
                        return HttpResponse::BadRequest().body("Invalid credentials.");
                    }
                    match encode_jwt(&app_state._config, user.id) {
                        Ok(token) => HttpResponse::Ok().body(token),
                        Err(err) => HttpResponse::InternalServerError().body(err.to_string()), //FIXME
                    }
                }
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()), //FIXME
            }
        },
        Ok(None) => HttpResponse::BadRequest().body("Invalid credentials..")
        ,
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()), //FIXME
    }
}