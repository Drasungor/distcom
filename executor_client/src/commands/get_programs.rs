use serde_derive::{Deserialize};

use crate::common::communication::EndpointResult;

#[derive(Deserialize, Debug)]
pub struct StoredProgram {
    pub organization_id: String,
    pub program_id: String,
    pub name: String,
    pub description: String,
    pub input_lock_timeout: i64,
}

#[derive(Deserialize, Debug)]
pub struct PagedPrograms {
    pub programs: Vec<StoredProgram>,
    pub total_elements_amount: i64,
}

pub async fn get_organization_programs(organization_id: String, limit: Option<u32>, page: Option<u32>) {

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

    let get_organization_programs_url = format!("http://localhost:8080/program/organization/{}", organization_id);

    // let response = reqwest::get("http://localhost:8080/account/organizations").await.expect("Error in get");
    let response = client.get(get_organization_programs_url).query(&params).send().await.expect("Error in get");

    // Ensure the request was successful (status code 200)
    if response.status().is_success() {
        let programs: EndpointResult<PagedPrograms> = response.json().await.expect("Error deserializing JSON");

        println!("get_organization_programs: {:?}", programs);
    } else {
        println!("Failed to download file: {}", response.status());
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
