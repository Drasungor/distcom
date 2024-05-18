
use serde_derive::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReturnedOrganization {
    pub organization_id: String,
    pub name: String,
    pub description: String,
}

pub fn print_organizations_list(organizations: &Vec<ReturnedOrganization>) {
    let mut index = 0;
    for organization in organizations {
        println!("Organization {}:", index);
        println!("\torganization_id: {}", organization.organization_id);
        println!("\tname: {}", organization.name);
        println!("\tdescription: {}", organization.description);
        index += 1;
    }
}