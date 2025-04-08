use std::collections::HashMap;

// src/global.rs
use crate::{models::errors::ClientErrorKind, services::client_manager::ClientManager};
use lazy_static::lazy_static;
use tokio::sync::RwLock;

lazy_static! {
    pub static ref GLOBAL_CLIENTS: RwLock<HashMap<String, ClientManager>> = RwLock::new(HashMap::new());
}

use crate::models::errors::ClientError;

pub async fn register_client(name: &str, addr: &str) -> Result<(), ClientError> {
    let manager = match ClientManager::new(addr).await {
        Ok(m) => m,
        Err(e) => {
            return Err(ClientError::new(
                500,
                ClientErrorKind::ConnectionFailed,
                &format!("Error creando cliente '{}': {}", name, e),
                "global::register_client",
            ))
        }
    };

    let mut map = GLOBAL_CLIENTS.write().await;
    map.insert(name.to_string(), manager);
    Ok(())
}

pub async fn get_client(name: &str) -> Option<ClientManager> {
    let map = GLOBAL_CLIENTS.read().await;
    map.get(name).cloned()
}