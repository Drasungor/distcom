use serde_derive::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EndpointResult<T> {
    pub status: String,
    pub data: T,
}

#[derive(Debug, Deserialize)]
pub struct EndpointError {
    pub status: String,
    pub error_code: String,
    pub error_message: String,
}