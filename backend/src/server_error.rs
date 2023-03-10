use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use diesel::result::{Error as DieselError};
use hive_lib::game_error::GameError;
use serde::Serialize;
use crate::{extractors::auth::AuthenticationError};
use thiserror::Error;

#[derive(Serialize)]
pub struct ErrorResponse {
    code: u16,
    message: String,
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Authentication error: {0}")]
    AuthenticationError(#[from] AuthenticationError),
    #[error("invalid field {field}: {reason}")]
    UserInputError {
        field: String,
        reason: String,
    },
    #[error("Hive game error: {0}")]
    GameError(#[from] GameError),
    #[error("Database error: {0}")]
    DatabaseError(#[from] DieselError),
    #[error("Unimplemented")]
    Unimplemented,
}

impl ResponseError for ServerError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::GameError(_) => StatusCode::BAD_REQUEST,
            Self::AuthenticationError(err) => match err {
                AuthenticationError::MissingToken => StatusCode::UNAUTHORIZED,
                AuthenticationError::Forbidden => StatusCode::FORBIDDEN,
                AuthenticationError::MalformedJWT(_) | AuthenticationError::MissingSubject  => StatusCode::BAD_REQUEST,
                AuthenticationError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                AuthenticationError::InvalidJWT(_) => StatusCode::UNAUTHORIZED,
            },
            Self::UserInputError { field: _, reason: _ } => StatusCode::BAD_REQUEST,
            Self::DatabaseError(err) => match err {
                DieselError::NotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            Self::Unimplemented => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        // TODO: don't send a message for 500s
        let error_response = ErrorResponse {
            code: status_code.as_u16(),
            message: self.to_string(),
        };
        HttpResponse::build(status_code).json(error_response)
    }
}
