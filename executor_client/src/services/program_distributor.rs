use serde_derive::{Deserialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use reqwest::{Client, RequestBuilder, Response};

use crate::{common::communication::EndpointResult, models::{returned_organization::ReturnedOrganization, returned_program::ReturnedProgram}, utils::compression::decompress_tar};

pub struct ProgramDistributorService {
    client: Client,
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

impl ProgramDistributorService {

    pub fn new() -> ProgramDistributorService {
        ProgramDistributorService {
            client: reqwest::Client::new(),
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


}



