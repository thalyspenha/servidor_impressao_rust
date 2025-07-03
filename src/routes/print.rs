use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use tracing::{info, error};

use crate::config::Config;
use crate::utils::{validar_jwt, detectar_tipo_arquivo};
use crate::services::printer::enviar_para_impressora;

#[post("/print")]
pub async fn print_handler(
    body: web::Bytes,
    req: HttpRequest,
    auth: BearerAuth,
    config: web::Data<Config>,
) -> impl Responder {
    let ip = req.peer_addr().map(|x| x.ip().to_string()).unwrap_or_else(|| "desconhecido".to_string());

    if let Err(e) = validar_jwt(auth.token(), &config.jwt_secret) {
        error!("Token inválido de {}: {}", ip, e);
        return HttpResponse::Unauthorized().body("Token inválido");
    }

    let tipo = detectar_tipo_arquivo(&body);
    if tipo.is_none() {
        return HttpResponse::BadRequest().body("Arquivo inválido");
    }

    match enviar_para_impressora(&body, tipo.unwrap(), &config.zebra_addr).await {
        Ok(_) => {
            info!("Arquivo enviado para a impressora com sucesso de {}", ip);
            HttpResponse::Ok().body("Impressão enviada com sucesso")
        },
        Err(e) => {
            error!("Erro de impressão de {}: {}", ip, e);
            HttpResponse::InternalServerError().body(format!("Erro: {}", e))
        }
    }
}

