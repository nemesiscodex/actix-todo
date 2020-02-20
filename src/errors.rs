use derive_more::Display;
use serde::Serialize;
use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};

#[derive(Display, Debug)]
pub enum AppError {
    #[display(fmt = "{}", _0)]
    DbError(String),
    NotModified,
    #[display(fmt = "{}", _0)]
    NotFound(String)
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub message: String
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::NotModified => StatusCode::NOT_MODIFIED
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(AppErrorResponse { message: self.to_string() })
    }
}
