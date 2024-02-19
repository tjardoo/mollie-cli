use std::fmt::{Debug, Display};

use colored::Colorize;
use mollie_api::errors::ApiError;

#[derive(Debug)]
pub enum CliError {
    EndpointError { url: String, status: u16 },
    ParseError,
}

impl From<ApiError> for CliError {
    fn from(error: ApiError) -> Self {
        match error {
            ApiError::EndpointError { url, status } => CliError::EndpointError {
                url: url.to_string(),
                status: status.as_u16(),
            },
            ApiError::ParseError(_) => CliError::ParseError,
        }
    }
}

impl Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::EndpointError { url, status } => {
                write!(f, "{} - {}", status, url.bold().red())
            }
            CliError::ParseError => write!(f, "Parse error"),
        }
    }
}
