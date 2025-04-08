use rust_rpc::{
    controllers::connections::handle_connection,
    global::client_conections::{get_client, register_client},
    models::errors::{ServerError, ServerErrorKind},
};
use std::{error::Error, process};
use tokio::{
    self,
    net::TcpListener,
    signal,
    time::{sleep, Duration},
};

async fn shutdown_signal() {
    signal::ctrl_c().await.expect("No se pudo escuchar ctrl+c");
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Err(e) = run().await {
        eprintln!("Error: {e}");
        if let Some(source) = e.source() {
            eprintln!("Caused by:{source}")
        }
        process::exit(1);
    }
    Ok(())
}

async fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    let listener = match TcpListener::bind("127.0.0.1:5000").await {
        Ok(listener) => {
            println!("Servidor escuchando en 127.0.0.1:5000");
            listener
        }
        Err(err) => {
            return Err(Box::new(ServerError::new(
                400,
                ServerErrorKind::BindError,
                &format!("Error al vincular el socket: {}", err),
                "run", // Puedes cambiar esto a la ubicación adecuada
            )));
        }
    };

    tokio::spawn(reconnect_loop());

    loop {
        tokio::select! {
            _ = shutdown_signal() => {
                println!("🛑 Shutdown recibido. Cerrando servidor con dignidad...");
                break;
            }

            accept_result = listener.accept() => {
                match accept_result {
                    Ok((socket, addr)) => {
                        println!("Conexión aceptada de {:?}", addr);
                        tokio::spawn(async move {
                            if let Err(err) = handle_connection(socket).await {
                                eprintln!("Error al manejar la conexión: {}", err);
                            }
                        });
                    }
                    Err(err) => {
                        eprintln!("Error al aceptar conexión: {}", err);
                    }
                }
            }
        }
    }
    Ok(())
}

async fn reconnect_loop() {
    let addr = "127.0.0.1:65432";
    let name = "python";

    loop {
        match get_client(name).await {
            Some(client) => {
                match client.ping().await {
                    Ok(_) => {
                        println!("✅ [{}] conexión viva", name);
                        sleep(Duration::from_secs(10)).await;
                    }
                    Err(e) => {
                        eprintln!("🔌 [{}] ping fallido: {}. Reintentando conexión...", name, e);
                        let _ = register_client(name, addr).await;
                        sleep(Duration::from_secs(5)).await;
                    }
                }
            }
            None => {
                eprintln!("⚠️ [{}] cliente no encontrado. Intentando conexión...", name);
                let _ = register_client(name, addr).await;
                sleep(Duration::from_secs(5)).await;
            }
        }
    }
}
