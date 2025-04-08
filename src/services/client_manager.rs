use std::sync::Arc;
use tokio::sync::Mutex;
use crate::{models::errors::{ClientError, ClientErrorKind}, services::tcp_client::TcpClient};

#[derive(Clone)]
pub struct ClientManager {
    client: Arc<Mutex<TcpClient>>,
}
impl ClientManager {
    /// Inicializa la conexión TCP y la almacena en un Arc<Mutex<TcpClient>>
    pub async fn new(addr: &str) -> Result<Self, ClientError> {
        match TcpClient::connect(addr).await {
            Ok(client) => Ok(ClientManager {
                client: Arc::new(Mutex::new(client)),
            }),
            Err(e) => Err(ClientError::new(
                500,
                ClientErrorKind::ConnectionFailed,
                &format!("Error en ClientManager al conectar cliente: {}", e),
                "ClientManager::new",
            )),
        }
    }

    /// Retorna una referencia clonable al cliente compartido.
    pub fn get_client(&self) -> Arc<Mutex<TcpClient>> {
        self.client.clone()
    }

    pub async fn lock(&self) -> tokio::sync::MutexGuard<'_, TcpClient> {
        self.client.lock().await
    }

    pub async fn ping(&self) -> Result<(), ClientError> {
        let mut client = self.client.lock().await;
        client.send_ping().await
    }
}
