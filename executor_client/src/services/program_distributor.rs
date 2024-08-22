use reqwest::multipart::Part;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_derive::Deserialize;
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

    #[allow(dead_code)]
    pub total_elements_amount: i64,
}


#[derive(Debug, Deserialize)]
pub struct PagedPrograms {
    pub programs: Vec<ReturnedProgram>,

    #[allow(dead_code)]
    pub total_elements_amount: i64,
}

#[derive(Debug, Serialize)]
pub struct UploadedProof {
    pub organization_id: String,
    pub program_id: String,
    pub input_group_id: String,
}

pub struct ProgramWithInputFiles {
    pub program_file_name: String,
    pub input_file_name: String,
}

impl ProgramDistributorService {

    pub fn new(base_url: String) -> ProgramDistributorService {
        ProgramDistributorService {
            client: reqwest::Client::new(),
            base_url,
        }
    }

    pub async fn get_organizations(&self, limit: Option<usize>, page: Option<usize>) -> Result<PagedOrganizations, EndpointError> {
        let mut params: Vec<(&str, usize)> = Vec::new();
        if let Some(limit_value) = limit {
            params.push(("limit", limit_value))
        }
        if let Some(page_value) = page {
            params.push(("page", page_value))
        }
        let get_organizations_url = format!("{}/account/organizations", self.base_url);
        let get_organizations_request = self.client.get(get_organizations_url).query(&params);
        let response = self.make_request_with_response_body::<PagedOrganizations>(get_organizations_request).await?;
        Ok(response.data)
    }

    pub async fn get_organization_programs(&self, organization_id: &String, limit: Option<usize>, page: Option<usize>) -> Result<PagedPrograms, EndpointError> {
        let mut params: Vec<(&str, usize)> = Vec::new();
        if let Some(limit_value) = limit {
            params.push(("limit", limit_value))
        }
        if let Some(page_value) = page {
            params.push(("page", page_value))
        }
        let get_organization_programs_url = format!("{}/program/organization/{}", self.base_url, organization_id);
        let get_organization_programs_request = self.client.get(get_organization_programs_url).query(&params);
        let response = self.make_request_with_response_body::<PagedPrograms>(get_organization_programs_request).await?;
        Ok(response.data)
    }
    
    pub async fn get_general_programs(&self, limit: Option<usize>, page: Option<usize>) -> Result<PagedPrograms, EndpointError> {
        let mut params: Vec<(&str, usize)> = Vec::new();
        if let Some(limit_value) = limit {
            params.push(("limit", limit_value))
        }
        if let Some(page_value) = page {
            params.push(("page", page_value))
        }
        let get_general_programs_url = format!("{}/program/all", self.base_url);
        let get_general_programs_request = self.client.get(get_general_programs_url).query(&params);
        let response = self.make_request_with_response_body::<PagedPrograms>(get_general_programs_request).await?;
        Ok(response.data)
    }
    
    pub async fn get_program_and_input_group(&self, program_id: &String) -> Result<ProgramWithInputFiles, EndpointError> {
        let get_program_and_input_group_url = format!("{}/program/program-and-inputs/{}", self.base_url, program_id);
        let received_bytes = self.make_request_with_file_response(self.client.get(get_program_and_input_group_url)).await?;

        // Open a file to write the downloaded content
        let mut file = File::create("downloaded_program_with_input.tar").expect("Error in file creation");
        file.write_all(received_bytes.as_ref()).expect("Errors in file write");
        decompress_tar("./downloaded_program_with_input.tar", "./program_with_input").expect("Error in downloaded file decompression");

        let mut csv_file_name: Option<String> = None;
        let mut tar_file_name: Option<String> = None;

        // We scan the folder for the program .tar file
        let folder_contents = fs::read_dir("./program_with_input").expect("Error in ");
        for entry in folder_contents {
            let unwrapped_entry = entry.expect("Error in folder entry processing");
            let path = unwrapped_entry.path();
            let entry_name = unwrapped_entry.file_name().into_string().expect("Error in converion from OsString to string");
            let path_string = path.to_str().expect("Error in conversion from path to string");
            if entry_name.contains(".tar") {
                // println!("tar path_string: {}", path_string);
                decompress_tar(path_string, "./src/runner/methods").expect("Error in code folder decompression");
                tar_file_name = Some(entry_name.clone());
            }
            if entry_name.contains(".csv") {
                csv_file_name = Some(entry_name.clone());
            }
        }
        Ok(ProgramWithInputFiles {
            input_file_name: csv_file_name.expect("No csv input file was received"),
            program_file_name: tar_file_name.expect("No tar program file was received"),
        })
    }

    pub async fn upload_proof(&self, proof_file_path: &Path, uploaded_proof_data: UploadedProof) -> Result<(), EndpointError> {
        let upload_proof_url = format!("{}/program/proof", self.base_url);
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

        let response = self.make_request_with_stream_upload_and_response_body::<()>(post_methods_request_builder).await;
        match response {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }

    // Since requests that are sending a stream cannot be cloned, and to repeat the request in case of a fail due to
    // invalid jwt error we need to have another request builder instance (since the send method consumes the variable),
    // we need the same request from request stored in request_clone but built without the try_clone function
    async fn make_request_with_stream_upload_and_response_body<T: DeserializeOwned>(&self, request: RequestBuilder
                ) -> Result<EndpointResult<T>, EndpointError> {
        self.make_request_with_response_body::<T>(request).await
    }

    async fn make_request_with_response_body<T: DeserializeOwned>(&self, request: RequestBuilder) -> Result<EndpointResult<T>, EndpointError> {

        let response = request.send().await.expect("Error in request making");
        let response_parse_result = Self::parse_response_with_response_body::<T>(response).await;
        match response_parse_result {
            Ok(good_response) => Ok(good_response),
            Err(error_response) => Err(error_response),
        }
    }

    async fn make_request_with_file_response(&self, request: RequestBuilder) -> Result<bytes::Bytes, EndpointError> {
        let response = request.send().await.expect("Error in request making");
        if response.status().is_success() {
            Ok(response.bytes().await.expect("Error while getting request bytes"))
        } else {
            let endpoint_response: EndpointError = response.json().await.expect("Error deserializing JSON");
            Err(endpoint_response)
        }
    }

    async fn parse_response_with_response_body<T: DeserializeOwned>(response: Response) -> Result<EndpointResult<T>, EndpointError> {
        if response.status().is_success() {
            let endpoint_response: EndpointResult<T> = response.json().await.expect("Error deserializing JSON");
            Ok(endpoint_response)
        } else {
            let endpoint_response: EndpointError = response.json().await.expect("Error deserializing JSON");
            Err(endpoint_response)
        }
    }

}



