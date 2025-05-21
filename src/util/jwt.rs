use chrono::Utc;
use jsonwebtoken::{decode, encode, errors::Error, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub created: i64,
    pub id: i32,
}

pub fn encode_jwt(config: &Config, user_id: i32) -> Result<String, Error> {
    let token = Token {
        created: Utc::now().timestamp(),
        id: user_id
    };
    encode(&Header::default(), &token, &EncodingKey::from_secret(config.jwt_secret.as_ref()))
}

pub fn decode_jwt(config: &Config, token: String) -> Result<TokenData<Token>, Error> {
    decode(
        &token,
        &DecodingKey::from_secret(config.jwt_secret.as_ref()),
        &Validation::default()
    )
}