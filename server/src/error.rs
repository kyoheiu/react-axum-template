use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::error;

#[derive(Debug)]
pub enum Error {
    Io(String),
    SomeError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printable = match self {
            Error::Io(s) => s,
            Error::SomeError => "Some error occured.",
        };
        write!(f, "{}", printable)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err.to_string())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let body = match self {
            Error::Io(s) => s,
            Error::SomeError => "Some error occured.".to_string(),
        };
        error!(body);
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
