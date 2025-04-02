# OxidLink

**OxidLink** es una aplicación escrita en Rust que facilita la comunicación entre múltiples servidores mediante conexiones TCP asíncronas. Utiliza Tokio para el manejo concurrente de conexiones y procesa solicitudes en formato JSON, permitiendo enrutar acciones específicas (por ejemplo, solicitudes dirigidas a un servidor Python) de manera eficiente.

## Características

- **Servidor TCP Asíncrono:** Implementado con Tokio, el servidor escucha en `0.0.0.0:5000` y atiende conexiones concurrentes.
- **Manejo de Solicitudes JSON:** Recibe, deserializa y procesa solicitudes estructuradas en JSON, gestionando tanto respuestas exitosas como errores.
- **Comunicación Inter-servidores:** Incluye un cliente TCP global para conectarse con un servidor Python en `127.0.0.1:65432`.
- **Manejo Centralizado de Errores:** Implementación de módulos y tipos de error personalizados para una depuración y respuesta de fallos consistentes.
- **Arquitectura Modular:** Separación en módulos para controladores, modelos, handlers y servicios, facilitando la escalabilidad y mantenibilidad del código.

## Estructura del Proyecto

La estructura básica del proyecto es la siguiente:

- **src/**
  - **lib.rs:** Declara los módulos principales del proyecto: `models`, `controllers`, `handlers`, `services` y `global`.
  - **main.rs:** Punto de entrada de la aplicación; configura el listener TCP, inicializa el cliente global y maneja las conexiones entrantes.
  - **controllers/**
    - **connections.rs:** Contiene la lógica para la lectura y escritura en el socket, y el manejo de la conexión.
    - **handle_request.rs:** Se encarga de la deserialización de solicitudes y del enrutamiento hacia los handlers correspondientes (por ejemplo, el handler para solicitudes Python).
  - **handlers/**
    - **python.rs:** (Módulo referenciado en el código) Procesa las solicitudes específicas para el servidor Python.
  - **models/**
    - **errors.rs:** Define los tipos y estructuras de error para solicitudes y errores del servidor.
    - **request.rs:** Estructura que representa la consulta/solicitud recibida en formato JSON.
  - **services/**
    - **tcp_client.rs:** Implementa el cliente TCP utilizado para conectar con el servidor Python.
  - **global/**
    - **client_conections.rs:** Gestiona la inicialización y acceso a la conexión TCP global hacia el servidor Python.

## Requisitos

- **Rust y Cargo:** Se recomienda tener instalada la versión estable más reciente de Rust.
- **Dependencias Principales:** Tokio, serde y serde_json, entre otras (ver el archivo `Cargo.toml` para más detalles).

## Cómo Ejecutar

1. **Clonar el repositorio:**
   ```bash
   git clone https://github.com/tu_usuario/oxidlink.git
   cd oxidlink


## Construir el proyecto:

- cargo build --release

## Ejecutar la aplicación:

- cargo run --release

- Al iniciar, el servidor escuchará en 0.0.0.0:5000 y se intentará inicializar la conexión global con el servidor Python en 127.0.0.1:65432.

## Licencia
 - Distribuido bajo la Licencia MIT. Consulta el archivo LICENSE para más detalles.