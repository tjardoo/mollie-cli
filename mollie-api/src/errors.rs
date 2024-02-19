use std::fmt::Debug;

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("Request error")]
    RequestError(#[from] reqwest::Error),
    #[error("Parse error")]
    ParseError(#[from] serde_json::Error),
}
