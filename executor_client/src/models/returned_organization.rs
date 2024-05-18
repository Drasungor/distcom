
use serde_derive::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReturnedOrganization {
    pub organization_id: String,
    pub name: String,
    pub description: String,
}
