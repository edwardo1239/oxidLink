use std::{error::Error, process};

use rust_rpc::{
    controllers::connections::handle_connection,
    global::client_conections::init_global_client_python,
    models::errors::{ServerError, ServerErrorKind}
};
use tokio::{self, net::TcpListener};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if let Err(e) = run().await {
        eprintln!("Error: {e}");
        if let Some(source) = e.source() {
            eprintln!("Caused by:{source}")
        }
        process::exit(1);
    }
    Ok(())
}

async fn run() -> Result<(), Box<dyn Error>> {
    let listener = match TcpListener::bind("0.0.0.0:5000").await {
        Ok(listener) => {
            println!("Servidor escuchando en 0.0.0.0:5000");
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

    // Inicializa la conexión global para el cliente.
    match init_global_client_python("127.0.0.1:65432").await {
        Ok(_) => println!("Conexión global inicializada exitosamente"),
        Err(e) => {
            eprintln!("Error al inicializar la conexión global: {}", e);
            return Err(e);
        }
    }

    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                println!("Conexión aceptada de {:?}", addr);
                tokio::spawn(async move {
                    if let Err(err) = handle_connection(socket).await {
                        eprintln!("Error al manejar la conexión: {}", err);
                    }
                });
            }
            Err(err) => {
                return Err(Box::new(ServerError::new(
                    400,
                    ServerErrorKind::AcceptError,
                    &format!("Error al aceptar la conexión: {}", err),
                    "run",
                )))
            }
        }
    }
}
