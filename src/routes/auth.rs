use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};

use crate::models::{Claims, TokenResponse};
use crate::config::Config;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[post("/auth")]
pub async fn auth_handler(
    data: web::Json<LoginRequest>,
    config: web::Data<Config>,
) -> impl Responder {
    if data.username == "!@palacio#$" && data.password == "$rNQJ5r@67yAgc3ip!X5" {
        let exp = (Utc::now() + Duration::hours(24)).timestamp() as usize;
        let claims = Claims {
            sub: data.username.clone(),
            exp,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
        )
        .unwrap();

        HttpResponse::Ok().json(TokenResponse { token })
    } else {
        HttpResponse::Unauthorized().body("Usuário ou senha inválidos")
    }
}