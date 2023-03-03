use axum::response::{IntoResponse, Response};
use hyper::StatusCode;

pub struct LoginError {
    message: String,
    code: StatusCode,
}

impl LoginError {
    pub fn new(message: &str, code: StatusCode) -> Self {
        let message = String::from(message);
        tracing::error!("LoginError={}", message);
        Self { message, code }
    }
}

impl<E> From<E> for LoginError
where
    E: Into<anyhow::Error> + std::fmt::Display,
{
    fn from(e: E) -> Self {
        tracing::error!("{}", e.to_string());
        LoginError {
            message: String::from("Un problème est survenu"),
            code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for LoginError {
    fn into_response(self) -> Response {
        (self.code, self.message).into_response()
    }
}
