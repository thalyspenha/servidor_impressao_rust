use std::{fs::File, io::Write, net::TcpStream, process::Command};
use tempfile::NamedTempFile;
use tracing::{info, error};

pub async fn enviar_para_impressora(content: &[u8], tipo: &str, printer_target: &str) -> Result<String, String> {
    info!("Requisição de impressão recebida de {printer_target} - tipo: {tipo}");

    match tipo {
        "zpl" | "txt" | "pdf" => {
            let file_path = NamedTempFile::new()
                .unwrap()
                .into_temp_path()
                .with_extension(tipo);
            let mut temp = File::create(&file_path).unwrap();
            temp.write_all(&content).unwrap();
            info!("Arquivo temporário criado em {:?}", file_path);

            if printer_target.contains(':') {
                info!("Conectando via TCP/IP em {}", printer_target);
                match TcpStream::connect(printer_target) {
                    Ok(mut stream) => {
                        if let Err(e) = stream.write_all(&content) {
                            error!("Erro ao enviar dados para impressora via TCP de {}: {}", printer_target, e);
                            return Err("Falha ao enviar para impressora".to_string());
                        }
                        info!("Arquivo enviado com sucesso via TCP para {}", printer_target);
                        return Ok("Enviado para impressora Zebra via TCP".to_string());
                    }
                    Err(e) => {
                        error!("Erro ao conectar na impressora {}: {}", printer_target, e);
                        return Err("Erro ao conectar na impressora".to_string());
                    }
                }
            } else {
                info!("Imprimindo localmente via nome '{}'", printer_target);
                match imprimir_local(printer_target, file_path.to_str().unwrap()) {
                    Ok(_) => {
                        info!("Impressão enviada para '{}'", printer_target);
                        return Ok("Enviado para a impressora".to_string());
                    }
                    Err(e) => {
                        error!("Erro ao imprimir localmente '{}': {}", printer_target, e);
                        return Err(format!("Erro ao imprimir: {}", e));
                    }
                }
            }
        }
        _ => {
            error!("Tipo de arquivo não suportado: {}", tipo);
            return Err("Tipo de arquivo não suportado".to_string());
        }
    }
}

fn imprimir_local(printer_name: &str, file_path: &str) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("cmd")
            .args(["/C", "print", "/D:".to_owned() + printer_name, file_path])
            .output();
        return match output {
            Ok(out) if out.status.success() => Ok(()),
            Ok(out) => Err(String::from_utf8_lossy(&out.stderr).to_string()),
            Err(e) => Err(e.to_string()),
        };
    }

    #[cfg(target_os = "macos")]
    {
        let output = Command::new("lp")
            .args(["-d", printer_name, file_path])
            .output();
        return match output {
            Ok(out) if out.status.success() => Ok(()),
            Ok(out) => Err(String::from_utf8_lossy(&out.stderr).to_string()),
            Err(e) => Err(e.to_string()),
        };
    }

    #[cfg(target_os = "linux")]
    {
        let lp_path = Command::new("which")
            .arg("lp")
            .output()
            .ok()
            .and_then(|o| {
                if o.status.success() {
                    Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
                } else {
                    None
                }
            });

        if let Some(_) = lp_path {
            let output = Command::new("lp")
                .args(["-d", printer_name, file_path])
                .output();
            return match output {
                Ok(out) if out.status.success() => Ok(()),
                Ok(out) => Err(String::from_utf8_lossy(&out.stderr).to_string()),
                Err(e) => Err(e.to_string()),
            };
        } else {
            return Err("Comando 'lp' não encontrado no sistema".to_string());
        }
    }
}