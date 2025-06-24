use actix_web::web;

pub mod auth;
pub mod print;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(auth::auth_handler);
    cfg.service(print::print_handler);
}
