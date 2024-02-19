use std::fmt::{Debug, Display};

use colored::Colorize;
use mollie_api::errors::ApiError;

#[derive(Debug)]
pub enum CliError {
    BadRequest(String),
    NotFound(String),
    UnprocessableEntity(String),
    ServerError(String),
    TooManyRequests(String),
    ParseError,
}

impl From<ApiError> for CliError {
    fn from(error: ApiError) -> Self {
        match error {
            ApiError::EndpointError { url, status } => match status.as_u16() {
                400 => CliError::BadRequest(url.to_string()),
                404 => CliError::NotFound(url.to_string()),
                422 => CliError::UnprocessableEntity(url.to_string()),
                429 => CliError::TooManyRequests(url.to_string()),
                500..=599 => CliError::ServerError(url.to_string()),
                _ => CliError::ServerError(url.to_string()),
            },
            ApiError::ParseError(_) => CliError::ParseError,
        }
    }
}

impl Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::BadRequest(url) => write!(f, "400 - Bad Request {}", url.bold().red()),
            CliError::NotFound(url) => write!(f, "404 - Not Found: {}", url.bold().red()),
            CliError::UnprocessableEntity(url) => {
                write!(f, "422 - Unprocessable Entity: {}", url.bold().red())
            }
            CliError::TooManyRequests(url) => {
                write!(f, "429 - Too Many Requests: {}", url.bold().red())
            }
            CliError::ServerError(url) => write!(f, "500 - Server Error: {}", url.bold().red()),
            CliError::ParseError => write!(f, "Parse error"),
        }
    }
}
