use crate::{
    handlers::python::handle_python_requests,
    models::{
        errors::{RequestError, RequestErrorKind},
        request::Query,
    },
};
use serde_json;

pub async fn serde_request(request: &str) -> Result<Query, RequestError> {
    if request.trim().is_empty() {
        return Err(RequestError::new(
            400,
            RequestErrorKind::EmptyAction,
            "La acción está vacía.",
            "controllers::handle_request::serde_request",
        ));
    }
    println!("{:?}", request);
    let query: Query = match serde_json::from_str::<Query>(request) {
        Ok(query) => {
            if query.action.is_empty() {
                return Err(RequestError::new(
                    422,
                    RequestErrorKind::EmptyAction,
                    "El campo 'action' está vacío.",
                    "Deseriacontrollers::handle_request::serde_requestlización",
                ));
            }
            query
        }
        Err(err) => {
            let (kind, message) = match err.classify() {
                serde_json::error::Category::Syntax => (
                    RequestErrorKind::InvalidJSON,
                    "El formato del JSON es inválido.",
                ),
                serde_json::error::Category::Data => (
                    RequestErrorKind::InvalidJSON,
                    "Los datos no coinciden con la estructura esperada.",
                ),
                _ => (
                    RequestErrorKind::InvalidJSON,
                    "Error desconocido en la deserialización.",
                ),
            };
            return Err(RequestError::new(
                400,
                kind,
                message,
                "controllers::handle_request::serde_request",
            ));
        }
    };
    Ok(query)
}

pub async fn handle_request_server(request: Query) -> Result<String, RequestError> {
    match request.server.as_str() {
        "python" => match handle_python_requests(request).await {
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
