use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, RequestBuilder, Response};
use reqwest::multipart::{self, Part};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::collections::HashMap;
use std::path::Path;
use rpassword;
use std::io;
use bytes::Bytes;
// use std::io::Write;

use crate::common::communication::{EndpointError, EndpointResult, AppErrorType};
use crate::common::user_interaction::get_input_string;
use crate::models::returned_input_group::ReturnedInputGroup;
use crate::models::returned_program::ReturnedProgram;
use crate::utils::compression::{compress_folder_contents, decompress_tar};

// TODO: check if we should add an attribute that stores the server's ip
pub struct ProgramDistributorService {
    base_url: String,
    jwt: Option<String>,
    client: Client,
}

#[derive(Debug, Deserialize)]
pub struct PagedPrograms {
    pub programs: Vec<ReturnedProgram>,
    pub total_elements_amount: i64,
}

#[derive(Debug, Deserialize)]
pub struct PagedProgramInputGroups {
    pub program_input_groups: Vec<ReturnedInputGroup>,
    pub total_elements_amount: i64,
}

// #[derive(Debug, Deserialize)]
// pub struct PagedProofs {
//     pub proofs: Vec<ReturnedProof>,
//     pub total_elements_amount: i64,
// }

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

#[derive(Deserialize, Debug)]
struct UploadedProgramReturnedData {
    pub program_id: String,
}

// UploadedProgramReturnedData, UploadedInputGroupReturnedData

#[derive(Deserialize, Debug)]
struct UploadedInputGroupReturnedData {
    pub input_group_id: String,
}


#[derive(Debug, Serialize)]
pub struct UploadedProgram {
    pub name: String,
    pub description: String,
    pub execution_timeout: i64
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

    pub async fn download_template_methods(&mut self, download_path: &Path) -> Result<(), EndpointError> {
        let get_template_url = format!("{}/program/template", self.base_url);
        let get_template_request_builder = self.client.get(get_template_url);
        let bytes = self.make_request_with_file_response(get_template_request_builder).await?;

        // TODO: handle this error correctly
        let download_path_str = download_path.to_str().expect("Error in get download path string");

        let downloaded_file_path = "./aux_files/downloaded_template.tar";
        let mut file = File::create(downloaded_file_path).expect("Error in file creation");
        file.write_all(bytes.as_ref()).expect("Errors in file write");
        decompress_tar(downloaded_file_path, download_path_str).expect("Error in downloaded file decompression");
        return Ok(());
    }

    pub async fn download_proof(&mut self, program_id: &str, input_group_id: &str, download_path: &Path) -> Result<(), EndpointError> {
        let get_proof_url = format!("{}/program/proof/{program_id}/{input_group_id}", self.base_url);
        let get_proof_request_builder = self.client.get(get_proof_url);
        let bytes = self.make_request_with_file_response(get_proof_request_builder).await?;

        // TODO: handle this error correctly
        let download_path_str = download_path.to_str().expect("Error in get download path string");

        let mut file = File::create(download_path_str).expect("Error in file creation");
        file.write_all(bytes.as_ref()).expect("Errors in file write");
        return Ok(());
    }

    pub async fn get_my_programs(&mut self, limit: Option<usize>, page: Option<usize>) -> Result<PagedPrograms, EndpointError> {
        let mut params: Vec<(&str, usize)> = Vec::new();
        if (limit.is_some()) {
            params.push(("limit", limit.unwrap()))
        }
        if (page.is_some()) {
            params.push(("page", page.unwrap()))
        }
        let get_my_programs_url = format!("{}/program/mine", self.base_url);
        let get_my_programs_request_builder = self.client.get(get_my_programs_url).query(&params);
        let get_my_programs_response = self.make_request_with_response_body::<PagedPrograms>(get_my_programs_request_builder).await?;
        return Ok(get_my_programs_response.data);
    }

    pub async fn get_program_proven_inputs(&mut self, program_id: &str, limit: Option<usize>, page: Option<usize>) -> Result<PagedProgramInputGroups, EndpointError> {
        let mut params: Vec<(&str, usize)> = Vec::new();
        if (limit.is_some()) {
            params.push(("limit", limit.unwrap()))
        }
        if (page.is_some()) {
            params.push(("page", page.unwrap()))
        }
        let get_my_programs_url = format!("{}/program/proofs/{program_id}", self.base_url);
        let get_my_programs_request_builder = self.client.get(get_my_programs_url).query(&params);
        let get_my_programs_response = self.make_request_with_response_body::<PagedProgramInputGroups>(get_my_programs_request_builder).await?;
        return Ok(get_my_programs_response.data);
    }

    pub async fn get_my_proven_programs(&mut self, limit: Option<usize>, page: Option<usize>) -> Result<PagedPrograms, EndpointError> {
        let mut params: Vec<(&str, usize)> = Vec::new();
        if (limit.is_some()) {
            params.push(("limit", limit.unwrap()))
        }
        if (page.is_some()) {
            params.push(("page", page.unwrap()))
        }
        let get_my_proven_programs_url = format!("{}/program/proofs", self.base_url);
        let get_my_proven_programs_request_builder = self.client.get(get_my_proven_programs_url).query(&params);
        let get_my_proven_programs_response = self.make_request_with_response_body::<PagedPrograms>(get_my_proven_programs_request_builder).await?;
        return Ok(get_my_proven_programs_response.data);
    }

    pub async fn upload_methods(&mut self, upload_folder_path: &Path, uploaded_program: UploadedProgram) -> Result<String, EndpointError> {
        let post_program_url = format!("{}/program/upload", self.base_url);
        let upload_folder_path_str = upload_folder_path.to_str().expect("Error in get download path string");
        let compressed_folder_path = "./aux_files/uploaded_methods.tar";
        compress_folder_contents(upload_folder_path_str, compressed_folder_path).expect("Error in methods folder compression");

        // Read the compressed file content
        let mut file = File::open(compressed_folder_path).expect("Error in opening compressed file");
        let mut file_content = Vec::new();
        file.read_to_end(&mut file_content).expect("Error in reading compressed file content");

        let serialized = serde_json::to_string(&uploaded_program).unwrap();

        let form = multipart::Form::new()
        .text("data", serialized.clone())
        .part("file", Part::bytes(file_content.clone()).file_name("uploaded_methods.tar"));
        let post_methods_request_builder = self.client.post(&post_program_url).multipart(form);
        
        let form_clone = multipart::Form::new()
        .text("data", serialized)
        .part("file", Part::bytes(file_content).file_name("uploaded_methods.tar"));
        let post_methods_request_builder_clone = self.client.post(&post_program_url).multipart(form_clone);

        // self.make_request_with_stream_upload_and_response_body::<()>(
        let uploaded_program_data = self.make_request_with_stream_upload_and_response_body::<UploadedProgramReturnedData>(
                                                                post_methods_request_builder, post_methods_request_builder_clone).await?;
        return Ok(uploaded_program_data.data.program_id);
    }

    // UploadedProgramReturnedData, UploadedInputGroupReturnedData


    pub async fn upload_input_group(&mut self, program_id: &str, uploaded_input_group_file_path: &Path) -> Result<String, EndpointError> {
        let post_program_input_group_url = format!("{}/program/inputs/{}", self.base_url, program_id);
        let uploaded_input_group_file_path_str = uploaded_input_group_file_path.to_str().expect("Error in get download path string");

        let mut file = File::open(uploaded_input_group_file_path_str).expect("Error in opening compressed file");
        let mut file_content = Vec::new();
        file.read_to_end(&mut file_content).expect("Error in reading compressed file content");

        let form = multipart::Form::new()
        .part("file", Part::bytes(file_content.clone()).file_name("program_input_group.csv"));
        let post_program_input_group_builder = self.client.post(&post_program_input_group_url).multipart(form);
        
        let form_clone = multipart::Form::new()
        .part("file", Part::bytes(file_content).file_name("program_input_group_clone.csv"));
        let post_program_input_group_builder_clone = self.client.post(&post_program_input_group_url).multipart(form_clone);

        // self.make_request_with_stream_upload_and_response_body::<()>(
        let uploaded_input_group_data = self.make_request_with_stream_upload_and_response_body::<UploadedInputGroupReturnedData>(
                                                post_program_input_group_builder, post_program_input_group_builder_clone).await?;
        return Ok(uploaded_input_group_data.data.input_group_id);
    }

    pub async fn download_program(&self, program_id: &str, download_path: &Path) {
        let get_program_url = format!("{}/program/{}", self.base_url, program_id);
        let response = reqwest::get(get_program_url).await.expect("Error in get");
    
        // Ensure the request was successful (status code 200)
        if response.status().is_success() {
            let file_path = "./aux_files/downloaded_program.tar";

            // Open a file to write the downloaded content
            let mut file = File::create(file_path).expect("Error in file creation");
            file.write_all(response.bytes().await.expect("Error in bytes get").as_ref()).expect("Errors in file write");
            decompress_tar(file_path, download_path.to_str().unwrap()).expect("Error in downloaded file decompression");
        } else {
            panic!("Failed to download file: {}", response.status());
        }
    }

    pub async fn mark_proof_as_invalid(&mut self, program_id: &str, input_group_id: &str) -> Result<(), EndpointError> {
        let patch_program_input_group_proof_url = format!("{}/program/proof/{program_id}/{input_group_id}", self.base_url);
        let patch_program_input_group_proof_request_builder = self.client.patch(patch_program_input_group_proof_url);
        self.make_request_with_response_body::<()>(patch_program_input_group_proof_request_builder).await?;
        return Ok(());
    }

    pub async fn confirm_proof_validity(&mut self, program_id: &str, input_group_id: &str) -> Result<(), EndpointError> {
        let patch_program_input_group_proof_url = format!("{}/program/proof/{program_id}/{input_group_id}", self.base_url);
        let patch_program_input_group_proof_request_builder = self.client.delete(patch_program_input_group_proof_url);
        self.make_request_with_response_body::<()>(patch_program_input_group_proof_request_builder).await?;
        return Ok(());
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
    
        let post_login_url = format!("{}/account/login", self.base_url);

        // TODO: Ensure the request was successful (status code 200)
        let response = self.client.post(post_login_url).json(&data).send().await.expect("Error in get");
        
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
        let post_token_refreshment_url = format!("{}/account/refresh-token", self.base_url);
        let response = self.client.post(post_token_refreshment_url).json(&data).send().await.
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

    async fn make_request_with_response_body<T: DeserializeOwned>(&mut self, request: RequestBuilder) -> Result<EndpointResult<T>, EndpointError> {
        let request_clone = request.try_clone().expect("Error while cloning request");
        return self.wrapper_make_request_with_response_body::<T>(request, request_clone).await;
    }

    // Since requests that are sending a stream cannot be cloned, and to repeat the request in case of a fail due to
    // invalid jwt error we need to have another request builder instance (since the send method consumes the variable),
    // we need the same request from request stored in request_clone but built without the try_clone function
    async fn make_request_with_stream_upload_and_response_body<T: DeserializeOwned>(&mut self, request: RequestBuilder, 
                                                              request_clone: RequestBuilder) -> Result<EndpointResult<T>, EndpointError> {
        return self.wrapper_make_request_with_response_body::<T>(request, request_clone).await;
    }

    async fn wrapper_make_request_with_response_body<T: DeserializeOwned>(&mut self, mut request: RequestBuilder, mut request_clone: RequestBuilder) -> Result<EndpointResult<T>, EndpointError> {
        let mut jwt_value = self.jwt.as_ref().expect("Jwt was not initialized").clone();
        let mut headers = HeaderMap::new();
        headers.insert("token", HeaderValue::from_str(&jwt_value).unwrap());
        request = request.headers(headers);

        let response = request.send().await.expect("Error in get");
        let response_parse_result = Self::parse_response_with_response_body::<T>(response).await;
        
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
                    // let response = request_clone.send().await.expect("Error in get");
                    jwt_value = self.jwt.as_ref().expect("Jwt was not initialized").clone();
                    headers = HeaderMap::new();
                    headers.insert("token", HeaderValue::from_str(&jwt_value).unwrap());
                    request_clone = request_clone.headers(headers);
                    let response = request_clone.send().await.expect("Error in get");
                    return Self::parse_response_with_response_body::<T>(response).await;
                } else {
                    return Err(error_response);
                }
            }
        }
    }

    async fn parse_response_with_response_body<T: DeserializeOwned>(response: Response) -> Result<EndpointResult<T>, EndpointError> {
        if response.status().is_success() {
            let endpoint_response: EndpointResult<T> = response.json().await.expect("Error deserializing JSON");
            return Ok(endpoint_response);
        } else {
            let endpoint_response: EndpointError = response.json().await.expect("Error deserializing JSON");
            return Err(endpoint_response);
        }
    }

    async fn make_request_with_file_response(&mut self, mut request: RequestBuilder) -> Result<Bytes, EndpointError> {
        let mut request_clone = request.try_clone().expect("Error while cloning request");
        
        let mut jwt_value = self.jwt.as_ref().expect("Jwt was not initialized").clone();
        let mut headers = HeaderMap::new();
        headers.insert("token", HeaderValue::from_str(&jwt_value).unwrap());
        request = request.headers(headers);

        let response = request.send().await.expect("Error in sent request");
        let response_parse_result = Self::parse_response_with_file_response(response).await;
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
                    jwt_value = self.jwt.as_ref().expect("Jwt was not initialized").clone();
                    headers = HeaderMap::new();
                    headers.insert("token", HeaderValue::from_str(&jwt_value).unwrap());
                    request_clone = request_clone.headers(headers);
                    let response = request_clone.send().await.expect("Error in get");
                    return Self::parse_response_with_file_response(response).await;
                } else {
                    return Err(error_response);
                }
            }
        }
    }

    async fn parse_response_with_file_response(response: Response) -> Result<Bytes, EndpointError> {
        if response.status().is_success() {
            // TODO: change the expect to proper error management, investigate possible sources of errors
            let bytes_response: Bytes = response.bytes().await.expect("Error while receiving bytes");
            return Ok(bytes_response);
        } else {
            let endpoint_response: EndpointError = response.json().await.expect("Error deserializing JSON");
            return Err(endpoint_response);
        }
    }

}

