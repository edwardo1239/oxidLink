use crate::{
    handlers::python::handle_python_requests,
    models::{errors::{RequestError, RequestErrorKind}, request::Query},
};
use serde_json::{self, error::Category};

pub async fn serde_request(request: &str) -> Result<Query, RequestError> {
    match serde_json::from_str::<Query>(request) {
        Ok(query) => Ok(query),
        Err(err) => {
            // Verificamos la categoría del error
            match err.classify() {
                Category::Eof => {

                    Err(RequestError::new(
                        400,
                        RequestErrorKind::EOFWhileParsing,
                        "JSON incompleto (EOF while parsing).",
                        "controllers::handle_request::serde_request",
                    ))
                }
                Category::Syntax => {
                    Err(RequestError::new(
                        400,
                        RequestErrorKind::InvalidJSON,
                        "El formato del JSON es inválido.",
                        "controllers::handle_request::serde_request",
                    ))
                }
                Category::Data => {
                    Err(RequestError::new(
                        400,
                        RequestErrorKind::InvalidJSON,
                        "Los datos no coinciden con la estructura esperada.",
                        "controllers::handle_request::serde_request",
                    ))
                }
                // Io o cualquier otro tipo que no sea Eof/Syntax/Data
                _ => {
                    Err(RequestError::new(
                        400,
                        RequestErrorKind::InvalidJSON,
                        "Error desconocido en la deserialización.",
                        "controllers::handle_request::serde_request",
                    ))
                }
            }
        }
    }
}

pub async fn handle_request_server(server: Query) -> Result<String, RequestError> {
    match server.server.as_str() {
        "python" => match handle_python_requests(server).await {
            Ok(data) => Ok(data),
            Err(err) => Err(err),
        },
        _ => Err(RequestError::new(
            404,
            RequestErrorKind::UnknownAction,
            "Acción no reconocida",
            "Deseriacontrollers::handle_request::route_request",
        )),
    }
}
