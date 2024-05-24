use serde_derive::{Serialize, Deserialize};

use crate::utils::jwt_helpers::GeneratedToken;

// Controller input models

#[derive(Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct ReceivedNewAccount {
    pub username: String,
    pub password: String,
    pub name: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct GetPagedOrganizations {
    pub limit: Option<i64>,
    pub page: Option<i64>,
    pub name_filter: Option<String>,
}

#[derive(Deserialize)]
pub struct TokenId {
    pub token_id: String,
}

#[derive(Deserialize)]
pub struct RefreshToken {
    pub refresh_token: String,
}

// Useful models

#[derive(Serialize)]
pub struct LoginTokens {
    pub basic_token: GeneratedToken,
    pub refresh_token: GeneratedToken,
}

// Controller output models

#[derive(Serialize)]
pub struct InitSession {
    pub token: String,
    pub refresh_token: String,
}

#[derive(Serialize)]
pub struct Token {
    pub basic_token: GeneratedToken,
}

#[derive(Serialize)]
pub struct ReturnedOrganization {
    pub organization_id: String,
    pub name: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct PagedOrganizations {
    pub organizations: Vec<ReturnedOrganization>,
    pub total_elements_amount: i64,
}