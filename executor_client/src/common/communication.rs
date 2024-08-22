use serde_derive::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EndpointResult<T> {
    #[allow(dead_code)]
    pub status: String,
    pub data: T,
}

#[derive(Debug, Deserialize)]
pub struct EndpointError {
    #[allow(dead_code)]
    pub status: String,

    #[allow(dead_code)]
    pub error_code: String,

    #[allow(dead_code)]
    pub error_message: String,
}