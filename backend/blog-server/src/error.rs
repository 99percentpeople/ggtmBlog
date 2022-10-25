use std::convert::Infallible;
use std::fmt::Debug;
use std::num::ParseIntError;

use actix_web::body::BoxBody;
use actix_web::error::Error as ActixError;
use actix_web::http::StatusCode;

use actix_web::rt::task::JoinError;
use actix_web::{error::ResponseError, HttpResponse};
use derive_more::{Display, Error};
use sea_orm::DbErr;

#[derive(Debug, Display, Error, Clone)]
pub enum ServerError {
    #[display(fmt = "database error: {}", field)]
    DataBaseError { field: String },
    #[display(fmt = "client error: {}", field)]
    ActixError { field: String, code: StatusCode },
    #[display(fmt = "not found error: {}", field)]
    NotFoundError { field: String },
    #[display(fmt = "infallible error: {}", field)]
    InfallibleError { field: String },
    #[display(fmt = "internal error: {}", field)]
    InternalError { field: String },
    #[display(fmt = "parse error: {}", field)]
    ParseError { field: String },
}
pub type ServerResult<T, E = ServerError> = std::result::Result<T, E>;

impl ResponseError for ServerError {
    fn status_code(&self) -> StatusCode {
        match self {
            ServerError::ActixError { code, .. } => code.clone(),
            ServerError::DataBaseError { .. } | ServerError::InfallibleError { .. } => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            ServerError::NotFoundError { .. } | ServerError::InternalError { .. } => {
                StatusCode::NOT_FOUND
            }
            ServerError::ParseError { .. } => StatusCode::BAD_REQUEST,
        }
    }
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}

impl From<ActixError> for ServerError {
    fn from(err: ActixError) -> Self {
        ServerError::ActixError {
            field: err.to_string(),
            code: err.error_response().status(),
        }
    }
}

impl From<DbErr> for ServerError {
    fn from(err: DbErr) -> Self {
        ServerError::DataBaseError {
            field: err.to_string(),
        }
    }
}

impl From<Infallible> for ServerError {
    fn from(err: Infallible) -> Self {
        ServerError::InfallibleError {
            field: err.to_string(),
        }
    }
}

impl From<std::io::Error> for ServerError {
    fn from(err: std::io::Error) -> Self {
        use std::io::ErrorKind;
        match err.kind() {
            ErrorKind::NotFound => ServerError::NotFoundError {
                field: err.to_string(),
            },
            _ => ServerError::InternalError {
                field: err.to_string(),
            },
        }
    }
}

impl From<ParseIntError> for ServerError {
    fn from(err: ParseIntError) -> Self {
        ServerError::ParseError {
            field: err.to_string(),
        }
    }
}

impl From<JoinError> for ServerError {
    fn from(err: JoinError) -> Self {
        ServerError::InternalError {
            field: err.to_string(),
        }
    }
}
