use actix_web::{ResponseError, HttpResponse};
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("json error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Qdrant error: {0}")]
    QdrantError(String),
}


impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::HttpError(_) => todo!(),
            Error::IoError(_) => todo!(),
            Error::JsonError(_) => todo!(),
            Error::QdrantError(_) => todo!(),
        }
    }

}