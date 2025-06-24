use std::env;
use actix_web::web;

#[derive(Clone)]
pub struct Config {
    pub jwt_secret: String,
    pub zebra_addr: String,
}

impl Config {
    pub fn from_env() -> Result<web::Data<Self>, &'static str> {
        Ok(web::Data::new(Config {
            jwt_secret: env::var("JWT_SECRET").map_err(|_| "JWT_SECRET ausente")?,
            zebra_addr: env::var("ZEBRA_PRINTER_ADDR").map_err(|_| "ZEBRA_PRINTER_ADDR ausente")?,
        }))
    }
}
