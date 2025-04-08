use crate::{
    global::client_conections::get_client, models::{errors::{RequestError, RequestErrorKind}, request::Query}
};

pub async fn handle_python_requests(request: Query) -> Result<String, RequestError> {
    let request_str = request.to_string();
    match get_client("python").await {
        Some(client) => {
            let mut tcp_client = client.lock().await;
            match tcp_client.send_message(&request_str).await {
                Ok(_) => (),
                Err(e) => {
                    return Err(RequestError::new(
                        404,
                        RequestErrorKind::InvalidAction,
                        &format!("No existe {:?} no existe", e),
                        "routes::lotes::route_functions_lotes",
                    ));
                }
            }
            let response = match tcp_client.receive_message().await {
                Ok(response) => response,
                Err(err) => {
                    return Err(RequestError::new(
                        405,
                        RequestErrorKind::InvalidData,
                        &format!("Error en la respuesta {:?}", err),
                        "routes::lotes::route_functions_lotes",
                    ))
                }
            };
            Ok(response)
        }
        None => {
            return Err(RequestError::new(
                404,
                RequestErrorKind::InvalidAction,
                &format!("No existe no existe"),
                "routes::lotes::route_functions_lotes",
            ));
        }
    }
}
