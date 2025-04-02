// src/global.rs
use std::error::Error;
use std::sync::Arc;
use tokio::sync::{Mutex, OnceCell};
use crate::services::tcp_client::TcpClient;

// Definimos la variable global que contendrá el cliente TCP.
pub static GLOBAL_CLIENT_PYTHON: OnceCell<Arc<Mutex<TcpClient>>> = OnceCell::const_new();

/// Inicializa la conexión global con el servidor TCP.
/// Esta función debe llamarse una única vez, por ejemplo, al inicio de la aplicación.
pub async fn init_global_client_python(addr: &str) -> Result<(), Box<dyn Error>> {
    let client = TcpClient::connect(addr).await?;
    GLOBAL_CLIENT_PYTHON
        .set(Arc::new(Mutex::new(client)))
        .map_err(|_| "El cliente global ya ha sido inicializado")?;
    Ok(())
}

/// Retorna una copia de la referencia al cliente global, si ya fue inicializado.
pub fn get_global_client_python() -> Option<Arc<Mutex<TcpClient>>> {
    GLOBAL_CLIENT_PYTHON.get().cloned()
}
