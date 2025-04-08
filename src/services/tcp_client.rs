use tokio::io::AsyncReadExt;
use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::models::errors::{ClientError, ClientErrorKind};

pub struct TcpClient {
    stream: TcpStream,
}

impl TcpClient {
    pub async fn connect(addr: &str) -> Result<Self, ClientError> {
        match TcpStream::connect(addr).await {
            Ok(stream) => Ok(TcpClient { stream }),
            Err(e) => Err(ClientError::new(
                500,
                ClientErrorKind::ConnectionFailed,
                &format!("No se pudo conectar a {}: {}", addr, e),
                "services::tcp_client::connect",
            )),
        }
    }

    pub async fn send_message(&mut self, message: &str) -> Result<(), ClientError> {
        match self.stream.write_all(message.as_bytes()).await {
            Ok(_) => Ok(()),
            Err(e) => Err(ClientError::new(
                500,
                ClientErrorKind::WriteFailed,
                &format!("No se pudo enviar el {}", e),
                "services::tcp_client::send_message",
            )),
        }
    }

    pub async fn receive_message(&mut self) -> Result<String, ClientError> {
        let mut buffer = [0; 1024];
        let n = match self.stream.read(&mut buffer).await {
            Ok(data) => data,
            Err(e) => return Err(ClientError::new(
                500,
                ClientErrorKind::Timeout,
                &format!("Error al recibir la respuesta: {}", e),
                "Services::tcp_client::receive_message",
            )),
        };

        let response = String::from_utf8_lossy(&buffer[..n]).to_string();
        println!("Respuesta recibida: {}", response);
        Ok(response)
    }

    pub async fn send_ping(&mut self) -> Result<(), ClientError> {
        match self.send_message("PING\n").await {
            Err(err) => Err(ClientError::new(
                500,
                ClientErrorKind::PingFailed,
                &format!("Error al enviar PING: {}", err),
                "services::tcp_client::send_ping",
            )),
            Ok(_) => match self.receive_message().await {
                Err(err) => Err(ClientError::new(
                    500,
                    ClientErrorKind::PingFailed,
                    &format!("Error al recibir PONG: {}", err),
                    "services::tcp_client::send_ping",
                )),
                Ok(response) if response.trim() == "PONG" => Ok(()),
                Ok(_) => Err(ClientError::new(
                    500,
                    ClientErrorKind::PingFailed,
                    "Respuesta inesperada al PING",
                    "services::tcp_client::send_ping",
                )),
            },
        }
    }
    
}
