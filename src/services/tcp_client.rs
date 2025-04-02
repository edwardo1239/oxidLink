use tokio::{io::AsyncWriteExt, net::TcpStream};
use tokio::io::{self, AsyncReadExt};
use std::io::ErrorKind;

use crate::models::errors::{ServerError, ServerErrorKind};

pub struct TcpClient {
    stream: TcpStream,
}

impl TcpClient {
    pub async fn connect(addr: &str) -> io::Result<Self> {
        let stream = match TcpStream::connect(addr).await {
            Ok(stream) => stream,
            Err(err) => {
                return Err(io::Error::new(
                    ErrorKind::Other,
                    ServerError::new(
                        500,
                        ServerErrorKind::BindError,
                        &format!("Error al conectar al servidor {}: {}", addr, err),
                        "TcpClient::connect",
                    ),
                ));
            }
        };

        Ok(TcpClient { stream })
    }

    pub async fn send_message(&mut self, message: &str) -> io::Result<()> {
        match self.stream.write_all(message.as_bytes()).await {
            Ok(_) => Ok(()),
            Err(err) => Err(io::Error::new(
                ErrorKind::Other,
                ServerError::new(
                    500,
                    ServerErrorKind::BindError,
                    &format!("Error al enviar mensaje: {}", err),
                    "Services::TcpClient::send_message",
                ),
            )),
        }
    }

    pub async fn receive_message(&mut self) -> io::Result<String>{
        let mut buffer = [0; 1024];
        let n = match self.stream.read(&mut buffer).await {
            Ok(data) => data,
            Err(err) => {
                return Err(io::Error::new(
                    ErrorKind::Other,
                    ServerError::new(
                        500,
                        ServerErrorKind::ResponseError,
                        &format!("Error al recibir la respuesta: {}", err),
                        "Services::TcpClient::receive_message",
                    ),
                ))
            }
        };
    
        let response = String::from_utf8_lossy(&buffer[..n]).to_string();
        Ok(response)
    }
}