use serde_derive::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EndpointResult<T> {
    pub status: String,
    pub data: T,
}
