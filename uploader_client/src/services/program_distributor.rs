use reqwest::{Client, RequestBuilder, Response};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;
use std::path::Path;
use rpassword;
use std::io;
// use std::io::Write;

use crate::common::communication::{EndpointError, EndpointResult, AppErrorType};
use crate::common::user_interaction::get_input_string;

// TODO: check if we should add an attribute that stores the server's ip
pub struct ProgramDistributorService {
    base_url: String,
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

    pub fn new(base_url: &str) -> ProgramDistributorService {
        ProgramDistributorService {
            base_url: base_url.to_string(),
            jwt: None,
            client: reqwest::Client::new(),
        }
    }

    pub async fn setup(&mut self) {
        self.get_jwt().await; 
    }

    pub async fn download_template_methods(&mut self) -> Result<(), EndpointError> {
        // make_request<T: DeserializeOwned>(&mut self, request: RequestBuilder) -> Result<EndpointResult<T>, EndpointError>
        let get_template_url = format!("{}/program/template", self.base_url);
        let get_template_request_builder = self.client.get(get_template_url);
        let request_result = self.make_request::<()>(get_template_request_builder).await;
        return match request_result {
            Ok(ok_result) => Ok(ok_result.data),
            Err(err) => Err(err),
        };
    }

    async fn interactive_login(&self) -> String {
        print!("Please enter your username: ");
        io::stdout().flush().unwrap();
        let username = get_input_string();
        let password = rpassword::prompt_password("Please enter your password: ").unwrap();
        let login_response = self.login(username, password).await;
        let refresh_token_file = File::create("./refresh_token.bin").expect("Error in refresh token file creation");
    
        // TODO: do an encryption for the refresh token storage, probably needs to ask for the users pc password, just like
        // in cellphones
        serde_json::to_writer(refresh_token_file, &login_response.data.refresh_token).expect("Error while saving refresh token object");
        return login_response.data.basic_token.token;
    }

    async fn login(&self, username: String, password: String) -> EndpointResult<ReceivedTokens> {
        let mut data = HashMap::new();
        data.insert("username", username);
        data.insert("password", password);
    
        // TODO: Ensure the request was successful (status code 200)
        let response = self.client.post("http://localhost:8080/account/login").json(&data).send().await.expect("Error in get");
        
        if response.status().is_success() {
            let login_response: EndpointResult<ReceivedTokens> = response.json().await.expect("Error deserializing JSON");
            return login_response;
        } else { 
            panic!("Error in login");
        }
    }

    async fn token_refreshment(&self, refresh_token: String) -> Result<EndpointResult<Token>, ()> {
        let mut data = HashMap::new();
        data.insert("refresh_token", refresh_token);
        let response = self.client.post("http://localhost:8080/account/refresh-token").json(&data).send().await.
                                    expect("Error in token refreshment endpoint call");
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
            returned_token = Some(self.interactive_login().await);
        }
        self.jwt = Some(returned_token.unwrap());
    }

    async fn make_request<T: DeserializeOwned>(&mut self, request: RequestBuilder) -> Result<EndpointResult<T>, EndpointError> {
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

