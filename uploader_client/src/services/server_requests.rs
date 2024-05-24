use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;

use crate::common::communication::EndpointResult;


#[derive(Debug, Deserialize, Serialize)]
pub struct Token {
    pub token_id: String,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct ReceivedTokens {
    pub basic_token: Token,
    pub refresh_token: Token,
}

// TODO: make it return a result that contains the struct instead of the array directly
pub async fn login(username: String, password: String) -> EndpointResult<ReceivedTokens> {

    // TODO: Check if the client should only be instanced once in the whole program execution
    let client = reqwest::Client::new();
    
    let mut data = HashMap::new();
    data.insert("username", username);
    data.insert("password", password);

    // TODO: Ensure the request was successful (status code 200)
    let response = client.post("http://localhost:8080/account/login").json(&data).send().await.expect("Error in get");
    
    if response.status().is_success() {
        let login_response: EndpointResult<ReceivedTokens> = response.json().await.expect("Error deserializing JSON");
        return login_response;
    } else { 
        panic!("Error in login");
    }
}

pub async fn token_refreshment(refresh_token: String) -> Result<EndpointResult<Token>, ()> {

    // TODO: Check if the client should only be instanced once in the whole program execution
    let client = reqwest::Client::new();
    
    let mut data = HashMap::new();
    data.insert("refresh_token", refresh_token);

    // TODO: Ensure the request was successful (status code 200)
    let response = client.post("http://localhost:8080/account/refresh-token").json(&data).send().await.expect("Error in get");
    
    if response.status().is_success() {
        let token_refreshment_response: EndpointResult<Token> = response.json().await.expect("Error deserializing JSON");
        return Ok(token_refreshment_response);
    } else {
        return Err(());
    }
}
