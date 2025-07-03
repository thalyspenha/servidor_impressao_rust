use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use crate::models::Claims;
use tracing::info;

pub fn validar_jwt(token: &str, secret: &str) -> Result<Claims, String> {
    info!("Validando token JWT...");
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map(|data| data.claims)
    .map_err(|err| format!("Token inválido: {}", err))
}

pub fn detectar_tipo_arquivo(bytes: &[u8]) -> Option<&'static str> {
    let s = String::from_utf8_lossy(bytes).to_lowercase();
    if s.contains("^xa") && s.contains("^xz") { Some("zpl") }
    else if s.contains("pdf") || bytes.starts_with(b"%PDF") { Some("pdf") }
    else if s.is_ascii() { Some("txt") }
    else { None }
}
