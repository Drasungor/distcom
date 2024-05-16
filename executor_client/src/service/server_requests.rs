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

pub async fn get_organizations(limit: Option<u32>, page: Option<u32>) {

    // let params: Vec<(&str, &str)> = Vec::new();
    let mut params: Vec<(&str, u32)> = Vec::new();

    if (limit.is_some()) {
        params.push(("limit", limit.unwrap()))
    }

    if (page.is_some()) {
        params.push(("page", page.unwrap()))
    }

    // TODO: Check if the client should only be instanced once in the whole program execution
    let client = reqwest::Client::new();

    println!("params: {:?}", params);

    // let response = reqwest::get("http://localhost:8080/account/organizations").await.expect("Error in get");
    let response = client.get("http://localhost:8080/account/organizations").query(&params).send().await.expect("Error in get");

    // Ensure the request was successful (status code 200)
    if response.status().is_success() {
        let get_organizations_response: EndpointResult<PagedOrganizations> = response.json().await.expect("Error deserializing JSON");

        let organizations = get_organizations_response.data.organizations;

        let organizations_amount = organizations.len();


        println!("Please select an organization or ask for a new page: {:?}", params);


        println!("get_organizations: {:?}", organizations);


    } else {
        println!("Failed to get organizations: {}", response.status());
    }
}