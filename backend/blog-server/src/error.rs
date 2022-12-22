use std::convert::Infallible;
use std::fmt::Debug;
use std::num::ParseIntError;

use actix_multipart::MultipartError;
use actix_web::body::BoxBody;
use actix_web::error::Error as ActixError;
use actix_web::http::StatusCode;

use actix_web::rt::task::JoinError;
use actix_web::{error::ResponseError, HttpResponse};
use derive_more::{Display, Error};
use sea_orm::DbErr;

#[derive(Debug, Display, Error, Clone)]
pub enum Error {
    #[display(fmt = "database error: {}", field)]
    DataBaseError { field: String },
    #[display(fmt = "server error: {}", field)]
    ServerError { field: String, code: StatusCode },
    #[display(fmt = "client error: {}", field)]
    ClientError { field: String },
}
pub type ServerResult<T, E = Error> = std::result::Result<T, E>;

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::ServerError { code, .. } => code.to_owned(),
            Error::DataBaseError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            Error::ClientError { .. } => StatusCode::BAD_REQUEST,
        }
    }
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}

impl From<ActixError> for Error {
    fn from(err: ActixError) -> Self {
        Error::ServerError {
            field: err.to_string(),
            code: err.error_response().status(),
        }
    }
}

impl From<DbErr> for Error {
    fn from(err: DbErr) -> Self {
        Error::DataBaseError {
            field: err.to_string(),
        }
    }
}

impl From<Infallible> for Error {
    fn from(err: Infallible) -> Self {
        Error::ClientError {
            field: err.to_string(),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        use std::io::ErrorKind;
        match err.kind() {
            ErrorKind::NotFound => Error::ClientError {
                field: err.to_string(),
            },
            _ => Error::ServerError {
                field: err.to_string(),
                code: StatusCode::INTERNAL_SERVER_ERROR,
            },
        }
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Error::ClientError {
            field: err.to_string(),
        }
    }
}

impl From<JoinError> for Error {
    fn from(err: JoinError) -> Self {
        Error::ServerError {
            field: err.to_string(),
            code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<MultipartError> for Error {
    fn from(err: MultipartError) -> Self {
        Error::ClientError {
            field: err.to_string(),
        }
    }
}
