use crate::common::communication::EndpointResult;
use serde_derive::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReturnedOrganization {
    pub organization_id: String,
    pub name: String,
    pub description: String,
}


#[derive(Debug, Deserialize)]
pub struct PagedOrganizations {
    pub organizations: Vec<ReturnedOrganization>,
    pub total_elements_amount: i64,
}

// TODO: make itreturn a result that contains the struct instead of the array directly
pub async fn get_organizations(limit: Option<u32>, page: Option<u32>) -> EndpointResult<PagedOrganizations> {
    let mut params: Vec<(&str, u32)> = Vec::new();

    if (limit.is_some()) {
        params.push(("limit", limit.unwrap()))
    }

    if (page.is_some()) {
        params.push(("page", page.unwrap()))
    }

    // TODO: Check if the client should only be instanced once in the whole program execution
    let client = reqwest::Client::new();
    
    // TODO: Ensure the request was successful (status code 200)
    let response = client.get("http://localhost:8080/account/organizations").query(&params).send().await.expect("Error in get");
    
    if response.status().is_success() {
        let get_organizations_response: EndpointResult<PagedOrganizations> = response.json().await.expect("Error deserializing JSON");
        return get_organizations_response;
    } else {
        panic!("Error in organizations get");
    }
}


#[derive(Debug, Deserialize)]
pub struct ReturnedProgram {
    pub organization_id: String,
    pub program_id: String,
    pub name: String,
    pub description: String,
    pub input_lock_timeout: i64,
}

#[derive(Debug, Deserialize)]
pub struct PagedPrograms {
    pub programs: Vec<ReturnedProgram>,
    pub total_elements_amount: i64,
}
pub async fn get_organization_programs(organization_id: &String, limit: Option<u32>, page: Option<u32>) -> EndpointResult<PagedPrograms> {
    let mut params: Vec<(&str, u32)> = Vec::new();

    if (limit.is_some()) {
        params.push(("limit", limit.unwrap()))
    }

    if (page.is_some()) {
        params.push(("page", page.unwrap()))
    }

    // TODO: Check if the client should only be instanced once in the whole program execution
    let client = reqwest::Client::new();
    let url = format!("http://localhost:8080/program/organization/{}", organization_id);

    let response = client.get(url).query(&params).send().await.expect("Error in get");

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

pub async fn get_general_programs(limit: Option<u32>, page: Option<u32>) {

    // let params: Vec<(&str, &str)> = Vec::new();
    let mut params: Vec<(&str, u32)> = Vec::new();

    if (limit.is_some()) {
        params.push(("limit", limit.unwrap()))
    }

    if (page.is_some()) {
        params.push(("limit", page.unwrap()))
    }

    // TODO: Check if the client should only be instanced once in the whole program execution
    let client = reqwest::Client::new();

    let get_programs_url = format!("http://localhost:8080/program/all");

    // let response = reqwest::get("http://localhost:8080/account/organizations").await.expect("Error in get");
    let response = client.get(get_programs_url).query(&params).send().await.expect("Error in get");

    // Ensure the request was successful (status code 200)
    if response.status().is_success() {
        let programs: EndpointResult<PagedPrograms> = response.json().await.expect("Error deserializing JSON");

        println!("get_organization_programs: {:?}", programs);
    } else {
        println!("Failed to download file: {}", response.status());
    }
}
