use std::fmt::{Debug, Display};

use colored::Colorize;
use mollie_api::errors::ApiError;

#[derive(Debug)]
pub enum CliError {
    NotFound(String),
    ParseError,
    RequestError(String),
    ServerError(String),
}

impl From<ApiError> for CliError {
    fn from(error: ApiError) -> Self {
        match error {
            ApiError::ParseError(_) => CliError::ParseError,
            ApiError::RequestError(request) => match request.status() {
                Some(status) => match status.as_u16() {
                    404 => CliError::NotFound(request.url().unwrap().to_string()),
                    500 => CliError::ServerError(request.url().unwrap().to_string()),
                    _ => CliError::RequestError(request.url().unwrap().to_string()),
                },
                None => CliError::RequestError(request.url().unwrap().to_string()),
            },
        }
    }
}

impl Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::NotFound(url) => write!(f, "Not found: {}", url.to_string().bold().red()),
            CliError::ParseError => write!(f, "Parse error"),
            CliError::RequestError(url) => {
                write!(f, "Request error: {}", url.to_string().bold().red())
            }
            CliError::ServerError(url) => {
                write!(f, "Server error: {}", url.to_string().bold().red())
            }
        }
    }
}
