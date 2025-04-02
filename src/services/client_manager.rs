use std::sync::Arc;
use tokio::sync::Mutex;
use std::error::Error;
use crate::services::tcp_client::TcpClient;

/// Un administrador que expone la conexión TCP compartida.
pub struct ClientManager {
    client: Arc<Mutex<TcpClient>>,
}

impl ClientManager {
    /// Inicializa la conexión TCP y la almacena en un Arc<Mutex<TcpClient>>
    pub async fn new(addr: &str) -> Result<Self, Box<dyn Error>> {
        let client = TcpClient::connect(addr).await?;
        Ok(ClientManager {
            client: Arc::new(Mutex::new(client)),
        })
    }

    /// Retorna una referencia clonable al cliente compartido.
    pub fn get_client(&self) -> Arc<Mutex<TcpClient>> {
        self.client.clone()
    }
}
