use std::fmt::{Debug, Display};

use reqwest::Url;

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    EndpointError {
        url: Url,
        status: reqwest::StatusCode,
    },
    ParseError(#[from] serde_json::Error),
}

impl From<reqwest::Error> for ApiError {
    fn from(error: reqwest::Error) -> Self {
        ApiError::EndpointError {
            url: error.url().unwrap().clone(),
            status: error.status().unwrap(),
        }
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::EndpointError { url, status } => {
                write!(f, "Endpoint error: {} - {}", url, status)
            }
            ApiError::ParseError(error) => write!(f, "Parse error: {}", error),
        }
    }
}
