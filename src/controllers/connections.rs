
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::{
    controllers::handle_request::{handle_request_server, serde_request},
    models::errors::{ServerError, ServerErrorKind},
};

pub async fn handle_connection(mut socket: TcpStream) -> Result<(), ServerError> {
    let mut buf = [0; 1024];

    loop {
        let n = match socket.read(&mut buf).await {
            Ok(n) => n,
            Err(err) => {
                return Err(ServerError::new(
                    400,
                    ServerErrorKind::ReadError,
                    &format!("Error al leer del socket: {}", err),
                    "controllers::connections::handle_connection",
                ));
            }
        };

        if n == 0 {
            // La conexión se ha cerrado
            println!("La conexión se ha cerrado.");
            return Ok(());
        }

        let request = String::from_utf8_lossy(&buf[..n]);

        match serde_request(&request).await {
            Ok(request) => {
                let response = match handle_request_server(request).await {
                    Ok(data) => data,
                    Err(err) => return Err(ServerError::new(
                        err.code().clone(), 
                        ServerErrorKind::InvalidRequest, 
                        &format!("{:?} => {}", err.kind(), err.message()), 
                        err.location(),
                    ))
                };
                match socket.write_all(response.as_bytes()).await {
                    Ok(_) => {
                        // Flush para asegurar que se envíe completamente
                        if let Err(flush_err) = socket.flush().await {
                            return Err(ServerError::new(
                                500,
                                ServerErrorKind::WriteError,
                                &format!(
                                    "Error al enviar la respuesta del servidor: {}",
                                    flush_err
                                ),
                                "controllers::connections::handle_connection",
                            ));
                        }

                        // Continuar esperando más datos
                        continue;
                    }
                    Err(write_err) => {
                        // Si no se puede escribir en el socket
                        return Err(ServerError::new(
                            500,
                            ServerErrorKind::WriteError,
                            &format!("No se pudo enviar el error al cliente: {}", write_err),
                            "controllers::connections::handle_connection",
                        ));
                    }
                }
            }
            Err(err) => {
                let error_response = serde_json::json!({
                    "error": true,
                    "code": err.code(),
                    "kind": format!("{:?}", err.kind()),
                    "message": err.message(),
                    "source": err.location()
                });

                // Convertir el error a JSON
                let error_json = match serde_json::to_string(&error_response) {
                    Ok(json) => json,
                    Err(_) => {
                        // Respaldo si la serialización falla
                        r#"{"error": true, "message": "Error interno de serialización"}"#
                            .to_string()
                    }
                };

                match socket.write_all(error_json.as_bytes()).await {
                    Ok(_) => {
                        // Flush para asegurar que se envíe completamente
                        if let Err(flush_err) = socket.flush().await {
                            return Err(ServerError::new(
                                500,
                                ServerErrorKind::WriteError,
                                &format!(
                                    "Error al hacer flush después de enviar error: {}",
                                    flush_err
                                ),
                                "controllers::connections::handle_connection",
                            ));
                        }

                        // Continuar esperando más datos
                        continue;
                    }
                    Err(write_err) => {
                        // Si no se puede escribir en el socket
                        return Err(ServerError::new(
                            500,
                            ServerErrorKind::WriteError,
                            &format!("No se pudo enviar el error al cliente: {}", write_err),
                            "controllers::connections::handle_connection",
                        ));
                    }
                }
            }
        };
    }
}
