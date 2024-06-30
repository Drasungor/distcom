use reqwest::multipart::Part;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_derive::{Deserialize};
use std::{fs, path::Path};
use std::fs::File;
use std::io::{Read, Write};
use reqwest::{multipart, Client, RequestBuilder, Response};

use crate::common::communication::EndpointError;
use crate::{common::communication::EndpointResult, models::{returned_organization::ReturnedOrganization, returned_program::ReturnedProgram}, utils::compression::decompress_tar};

pub struct ProgramDistributorService {
    client: Client,
    base_url: String,
}


#[derive(Debug, Deserialize)]
pub struct PagedOrganizations {
    pub organizations: Vec<ReturnedOrganization>,
    pub total_elements_amount: i64,
}


#[derive(Debug, Deserialize)]
pub struct PagedPrograms {
    pub programs: Vec<ReturnedProgram>,
    pub total_elements_amount: i64,
}

#[derive(Debug, Serialize)]
pub struct UploadedProof {
    pub organization_id: String,
    pub program_id: String,
    pub input_group_id: String,
}

impl ProgramDistributorService {

    pub fn new(base_url: String) -> ProgramDistributorService {
        ProgramDistributorService {
            client: reqwest::Client::new(),
            base_url,
        }
    }

    // TODO: make itreturn a result that contains the struct instead of the array directly
    pub async fn get_organizations(&self, limit: Option<usize>, page: Option<usize>) -> EndpointResult<PagedOrganizations> {
        let mut params: Vec<(&str, usize)> = Vec::new();

        if (limit.is_some()) {
            params.push(("limit", limit.unwrap()))
        }

        if (page.is_some()) {
            params.push(("page", page.unwrap()))
        }

        // TODO: Ensure the request was successful (status code 200)
        let response = self.client.get("http://localhost:8080/account/organizations").query(&params).send().await.expect("Error in get");
        
        if response.status().is_success() {
            let get_organizations_response: EndpointResult<PagedOrganizations> = response.json().await.expect("Error deserializing JSON");
            return get_organizations_response;
        } else {
            panic!("Error in organizations get");
        }
    }

    pub async fn get_organization_programs(&self, organization_id: &String, limit: Option<usize>, page: Option<usize>) -> EndpointResult<PagedPrograms> {
        let mut params: Vec<(&str, usize)> = Vec::new();
        if (limit.is_some()) {
            params.push(("limit", limit.unwrap()))
        }
        if (page.is_some()) {
            params.push(("page", page.unwrap()))
        }
        let url = format!("http://localhost:8080/program/organization/{}", organization_id);
        let response = self.client.get(url).query(&params).send().await.expect("Error in get");
    
        // Ensure the request was successful (status code 200)
        if response.status().is_success() {
            let get_organization_programs_response: EndpointResult<PagedPrograms> = response.json().await.expect("Error deserializing JSON");
            // let programs = get_organization_programs_response.data.programs;
            // let programs_amount = programs.len();
            
            return get_organization_programs_response;
        } else {
            panic!("Error in programs get");
        }
    }
    
    pub async fn get_general_programs(&self, limit: Option<usize>, page: Option<usize>) -> EndpointResult<PagedPrograms> {
        let mut params: Vec<(&str, usize)> = Vec::new();
        if (limit.is_some()) {
            params.push(("limit", limit.unwrap()))
        }
        if (page.is_some()) {
            params.push(("page", page.unwrap()))
        }
    
        let get_programs_url = format!("http://localhost:8080/program/all");
    
        // let response = reqwest::get("http://localhost:8080/account/organizations").await.expect("Error in get");
        let response = self.client.get(get_programs_url).query(&params).send().await.expect("Error in get");
    
        // Ensure the request was successful (status code 200)
        if response.status().is_success() {
            let programs: EndpointResult<PagedPrograms> = response.json().await.expect("Error deserializing JSON");
            return programs;
        } else {
            panic!("Error in programs get: {:?}", response);
        }
    }
    
    pub async fn get_program_and_input_group(&self, program_id: &String) -> String {
        let request_url = format!("http://localhost:8080/program/program-and-inputs/{}", program_id);
        let response = reqwest::get(request_url).await.expect("Error in get");
    
        // Ensure the request was successful (status code 200)
        if response.status().is_success() {
            // Open a file to write the downloaded content
            let mut file = File::create("downloaded_program_with_input.tar").expect("Error in file creation");
            file.write_all(response.bytes().await.expect("Error in bytes get").as_ref()).expect("Errors in file write");
            decompress_tar("./downloaded_program_with_input.tar", "./program_with_input").expect("Error in downloaded file decompression");
    
            let mut csv_file_name: Option<String> = None;
    
            // We scan the folder for the program .tar file
            let folder_contents = fs::read_dir("./program_with_input").expect("Error in ");
            for entry in folder_contents {
                let unwrapped_entry = entry.expect("Error in folder entry processing");
                let path = unwrapped_entry.path();
                let entry_name = unwrapped_entry.file_name().into_string().expect("Error in converion from OsString to string");
                let path_string = path.to_str().expect("Error in conversion from path to string");
                if (entry_name.contains(".tar")) {
                    println!("tar path_string: {}", path_string);
                    decompress_tar(path_string, "./src/runner/methods").expect("Error in code folder decompression");
                }
                if (entry_name.contains(".csv")) {
                    csv_file_name = Some(entry_name);
                    // println!("tar path_string: {}", path_string);
                    // decompress_tar(path_string, "./src/runner/methods").expect("Error in code folder decompression");
                }
            }
    
            return csv_file_name.expect("No csv file was received");
            // let output = Command::new("cargo")
            // .arg("run")
            // .current_dir("./src/runner")
            // .output()
            // .expect("Failed to execute child program");
        } else {
            panic!("Failed to download file: {}", response.status());
        }
    }

    pub async fn upload_proof(&mut self, proof_file_path: &Path, uploaded_proof_data: UploadedProof) -> Result<(), EndpointError> {
        let upload_proof_url = format!("{}/program/upload", self.base_url);
        let proof_file_path_str = proof_file_path.to_str().expect("Error in get proof path string");

        // Read the proof file content
        let mut file = File::open(proof_file_path_str).expect("Error opening proof file");
        let mut file_content = Vec::new();
        file.read_to_end(&mut file_content).expect("Error reading proof file content");

        let serialized_proof_args = serde_json::to_string(&uploaded_proof_data).unwrap();

        let form = multipart::Form::new()
        .text("data", serialized_proof_args.clone())
        .part("file", Part::bytes(file_content.clone()).file_name("uploaded_methods.tar"));
        let post_methods_request_builder = self.client.post(&upload_proof_url).multipart(form);
        
        let form_clone = multipart::Form::new()
        .text("data", serialized_proof_args)
        .part("file", Part::bytes(file_content).file_name("uploaded_methods.tar"));
        let post_methods_request_builder_clone = self.client.post(&upload_proof_url).multipart(form_clone);

        let response = self.make_request_with_stream_upload_and_response_body::<()>(
                                                                post_methods_request_builder, post_methods_request_builder_clone).await;
        return match response {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }

        // Since requests that are sending a stream cannot be cloned, and to repeat the request in case of a fail due to
    // invalid jwt error we need to have another request builder instance (since the send method consumes the variable),
    // we need the same request from request stored in request_clone but built without the try_clone function
    async fn make_request_with_stream_upload_and_response_body<T: DeserializeOwned>(&mut self, request: RequestBuilder, 
                                                              request_clone: RequestBuilder) -> Result<EndpointResult<T>, EndpointError> {
        return self.wrapper_make_request_with_response_body::<T>(request, request_clone).await;
    }

    async fn wrapper_make_request_with_response_body<T: DeserializeOwned>(&mut self, mut request: RequestBuilder, mut request_clone: RequestBuilder) -> Result<EndpointResult<T>, EndpointError> {

        let response = request.send().await.expect("Error in get");
        let response_parse_result = Self::parse_response_with_response_body::<T>(response).await;
        return match response_parse_result {
            Ok(good_response) => Ok(good_response),
            Err(error_response) => Err(error_response),
            // Err(error_response) => {
            //     let invalid_variant: Result<AppErrorType, String> = "InvalidVariant".parse();
            //     let error_type = match invalid_variant {
            //         Ok(v) => v,
            //         Err(e) => panic!("Received unknown error type: \"{}\" with message \"{}\"", e, error_response.error_message),
            //     };
            //     if (error_type == AppErrorType::InvalidToken) {
            //         self.get_jwt().await;
            //         // let response = request_clone.send().await.expect("Error in get");
            //         jwt_value = self.jwt.as_ref().expect("Jwt was not initialized").clone();
            //         headers = HeaderMap::new();
            //         headers.insert("token", HeaderValue::from_str(&jwt_value).unwrap());
            //         request_clone = request_clone.headers(headers);
            //         let response = request_clone.send().await.expect("Error in get");
            //         return Self::parse_response_with_response_body::<T>(response).await;
            //     } else {
            //         return Err(error_response);
            //     }
            // }
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

}



