use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::state::AppState;

#[derive(Serialize, Deserialize)]
pub struct Claims{
    pub sub: String,
    pub username : String,
    pub exp: usize,
}

pub fn create_jwt(id: uuid::Uuid, username: String, state : &AppState) -> String {
    
    let claims = Claims {
        sub: id.to_string(),
        username,
        exp: chrono::Utc::now()
            .checked_add_signed(chrono::Duration::days(7))
            .unwrap()
            .timestamp() as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(state.jwt_secret.as_bytes())).unwrap()
}

pub fn decode_jwt(token: &str, state : &AppState) -> Result<Claims, jsonwebtoken::errors::Error> {
    jsonwebtoken::decode(token, &DecodingKey::from_secret(state.jwt_secret.as_bytes()), &Validation::default())
        .map(|data| data.claims)
}