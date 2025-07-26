use std::fmt::Display;

use actix_web::{
    HttpResponse, ResponseError,
    http::{StatusCode, header::ContentType},
};
use serde::Serialize;

pub type Error = Box<(dyn std::error::Error + 'static)>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize, Default)]
pub struct ApiError {
    status: u16,
    message: String,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl From<Error> for ApiError {
    fn from(_: Error) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: String::from("Deu erro"),
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
            .insert_header(ContentType::html())
            .body(serde_json::to_string(self).unwrap())
    }
}
