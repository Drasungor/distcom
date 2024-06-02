use reqwest::{Client, RequestBuilder, Response};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;
use std::path::Path;

use crate::common::communication::{EndpointError, EndpointResult, AppErrorType};
use crate::common::user_interaction::get_input_string;

// TODO: check if we should add an attribute that stores the server's ip
pub struct ProgramDistributorService {
    jwt: Option<String>,
    client: Client,
}


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


impl ProgramDistributorService {

    pub fn new() -> ProgramDistributorService {
        ProgramDistributorService {
            jwt: None,
            client: reqwest::Client::new(),
        }
    }

    pub async fn setup(&mut self) {
        self.get_jwt().await; 
    }

    async fn interactive_login() -> String {
        println!("Please enter your username:");
        let username = get_input_string();
        
        println!("Please enter your password:");
        let password = get_input_string();
        
        let login_response = Self::login(username, password).await;
    
        let refresh_token_file = File::create("./refresh_token.bin").expect("Error in refresh token file creation");
    
        // TODO: do an encryption for the refresh token storage, probably needs to ask for the users pc password, just like
        // in cellphones
        serde_json::to_writer(refresh_token_file, &login_response.data.refresh_token).expect("Error while saving refresh token object");
        return login_response.data.basic_token.token;
    }

    // TODO: make it return a result that contains the struct instead of the array directly
    async fn login(username: String, password: String) -> EndpointResult<ReceivedTokens> {

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

    async fn token_refreshment(&self, refresh_token: String) -> Result<EndpointResult<Token>, ()> {
    
        // let client = reqwest::Client::new();
        
        let mut data = HashMap::new();
        data.insert("refresh_token", refresh_token);
    
        // TODO: Ensure the request was successful (status code 200)
        let response = self.client.post("http://localhost:8080/account/refresh-token").json(&data).send().await.expect("Error in get");
        
        if response.status().is_success() {
            let token_refreshment_response: EndpointResult<Token> = response.json().await.expect("Error deserializing JSON");
            return Ok(token_refreshment_response);
        } else {
            return Err(());
        }
    }

    async fn get_jwt(&mut self) {
        let mut should_log_in = false;
        let path = Path::new("./refresh_token.bin");
        let mut returned_token: Option<String> = None;
    
        if path.exists() {
            let refresh_token_file = File::open("./refresh_token.bin").expect("Error in refresh token file creation");
            let refresh_token: Token = serde_json::from_reader(refresh_token_file).expect("Error in token object deserialization");
    
            // TODO: add error management to token_refreshment function and also call login if the token refreshment fails
            let jwt_result = self.token_refreshment(refresh_token.token).await;
    
            if (jwt_result.is_ok()) {
                returned_token = Some(jwt_result.unwrap().data.token);
            } else {
                should_log_in = true;
            }
        } else {
            should_log_in = true;
        }
        if (should_log_in) {
            returned_token = Some(Self::interactive_login().await);
        }

        self.jwt = Some(returned_token.unwrap());
        // return returned_token.unwrap();
    }

    async fn make_request<T: DeserializeOwned>(&mut self, request: RequestBuilder) -> Result<EndpointResult<T>, EndpointError> {
        // let response = self.client.send().await.expect("Error in get");
        let request_clone = request.try_clone().expect("Error while cloning request");
        let response = request.send().await.expect("Error in get");
        let response_parse_result = Self::parse_response::<T>(response).await;

        return match response_parse_result {
            Ok(good_response) => Ok(good_response),
            Err(error_response) => {
                let invalid_variant: Result<AppErrorType, String> = "InvalidVariant".parse();
                let error_type = match invalid_variant {
                    Ok(v) => v,
                    Err(e) => panic!("Received unknown error type: \"{}\" with message \"{}\"", e, error_response.error_message),
                };
                if (error_type == AppErrorType::InvalidToken) {
                    self.get_jwt().await;
                    let response = request_clone.send().await.expect("Error in get");
                    return Self::parse_response::<T>(response).await;
                } else {
                    return Err(error_response);
                }
            }
        }
    }

    async fn parse_response<T: DeserializeOwned>(response: Response) -> Result<EndpointResult<T>, EndpointError> {
        if response.status().is_success() {
            let endpoint_response: EndpointResult<T> = response.json().await.expect("Error deserializing JSON");
            return Ok(endpoint_response);
        } else {
            let endpoint_response: EndpointError = response.json().await.expect("Error deserializing JSON");
            return Err(endpoint_response);
        }
    }

}

