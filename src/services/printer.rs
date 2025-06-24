use std::{fs::File, io::Write, net::TcpStream, process::Command};
use tempfile::NamedTempFile;

pub async fn enviar_para_impressora(content: &[u8], tipo: &str, printer_target: &str, client_ip: &str) -> Result<String, String> {
    match tipo {
        "zpl" | "txt" | "pdf" => {
            // Cria arquivo temporário com a extensão apropriada
            let file_path = NamedTempFile::new()
                .unwrap()
                .into_temp_path()
                .with_extension(tipo);
            let mut temp = File::create(&file_path).unwrap();
            temp.write_all(&content).unwrap();

            if printer_target.contains(':') {
                // Impressora por IP:porta (modo TCP/IP)
                match TcpStream::connect(printer_target) {
                    Ok(mut stream) => {
                        if let Err(e) = stream.write_all(&content) {
                            println!("Falha ao enviar para impressora: {}", client_ip);
                            return Err("Falha ao enviar para impressora".to_string());
                        }
                        println!("Enviado para impressora Zebra via {}", client_ip);
                        return Ok("Enviado para impressora Zebra via".to_string());
                    }
                    Err(e) => {
                        println!("Erro ao conectar na impressora: {}", client_ip);
                        return Err("Erro ao conectar na impressora".to_string());
                    }
                }
            } else {
                // Impressora local por nome
                match imprimir_local(printer_target, file_path.to_str().unwrap()) {
                    Ok(_) => {
                        println!("Enviado para a impressora '{}'", client_ip);
                        return Ok("Enviado para a impressora".to_string());
                    }
                    Err(e) => {
                        println!("Erro ao imprimir: {}", client_ip);
                        return Err("Erro ao imprimir:".to_string());
                    }
                }
            }
        }
        _ => return Err("Tipo de arquivo não suportado".to_string()),
    }
    // match tipo {
    //     "zpl" | "txt" => {
    //         TcpStream::connect(zebra_addr)
    //             .and_then(|mut s| s.write_all(conteudo))
    //             .map_err(|e| format!("Erro TCP: {}", e))?;
    //     }
    //     "pdf" => {
    //         let temp_path = NamedTempFile::new()
    //             .map_err(|e| e.to_string())?
    //             .into_temp_path()
    //             .with_extension("pdf");
    //         File::create(&temp_path)
    //             .and_then(|mut f| f.write_all(conteudo))
    //             .map_err(|e| format!("Erro ao gravar PDF: {}", e))?;

    //         #[cfg(target_os = "windows")]
    //         let output = Command::new("cmd")
    //             .args(["/C", "start", "/min", "", temp_path.to_str().unwrap()])
    //             .output();

    //         #[cfg(not(target_os = "windows"))]
    //         let output = Command::new("lp")
    //             .arg(temp_path.to_str().unwrap())
    //             .output();

    //         if let Err(e) = output {
    //             return Err(format!("Erro ao imprimir PDF: {}", e));
    //         }
    //     }
    //     _ => return Err("Tipo de arquivo não suportado".to_string()),
    // }

    //Ok(())
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