use rust_rpc::{
    controllers::connections::handle_connection,
    global::client_conections::init_global_client_python,
    models::errors::{ServerError, ServerErrorKind},
};
use std::{error::Error, process};
use tokio::{
    self,
    net::TcpListener,
    time::{sleep, Duration},
};

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

    tokio::spawn(reconnect_loop());

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

// Función que ejecuta la lógica de reconexión fuera del tokio::spawn
async fn reconnect_loop() {
    loop {
        let result = try_init_connection().await;

        if result {
            println!("✅ Conexión global inicializada exitosamente");
            break;
        } else {
            println!("⚠️ Reintentando conexión en 10 segundos...");
            sleep(Duration::from_secs(10)).await;
        }
    }
}

// Función que maneja el error sin dejar que llegue a tokio::spawn
async fn try_init_connection() -> bool {
    match init_global_client_python("127.0.0.1:65432").await {
        Ok(_) => true,
        Err(e) => {
            // Convertimos el error a string aquí y no lo dejamos escapar
            eprintln!("⚠️ Error al inicializar la conexión global: {}", e);
            false
        }
    }
}
