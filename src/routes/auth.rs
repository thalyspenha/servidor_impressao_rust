use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};
use tracing::info;

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
    info!("Tentativa de login de: {}", data.username);

    if data.username == "admin" && data.password == "1234" {
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
