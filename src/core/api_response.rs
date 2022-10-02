use actix_web::{get, web, HttpResponse, http::StatusCode};
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct ErrorMessage {
    pub message: String,
}

pub(crate) fn pack<T: Serialize>(result: Result<T, String>) -> HttpResponse {
    match result {
        Ok(data) => HttpResponse::Ok().status(StatusCode::OK).json(data),
        Err(error_message) => HttpResponse::ExpectationFailed().json(ErrorMessage{message: error_message}),
    }
}
