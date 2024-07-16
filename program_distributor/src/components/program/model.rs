use serde_derive::{Serialize, Deserialize};

use crate::utils::jwt_helpers::GeneratedToken;

use super::db_models::{program::StoredProgram, program_input_group::ProgramInputGroup};

// Controller input models

#[derive(Deserialize)]
pub struct UploadProgram {
    // Amount of seconds that will be waited before considering a requested program-input duo as abandoned
    pub name: String,
    pub description: String,
    pub execution_timeout: i64,
}

#[derive(Deserialize)]
pub struct UploadProof {
    pub organization_id: String,
    pub program_id: String,
    pub input_group_id: String,
}

#[derive(Deserialize)]
pub struct GetPagedPrograms {
    pub limit: Option<i64>,
    pub page: Option<i64>,
    pub name_filter: Option<String>,
}

// Useful models

// Controller output models

#[derive(Serialize, Debug)]
pub struct PagedPrograms {
    pub programs: Vec<StoredProgram>,
    pub total_elements_amount: i64,
}

#[derive(Serialize, Debug)]
pub struct PagedProgramInputGroups {
    pub program_input_groups: Vec<ProgramInputGroup>,
    pub total_elements_amount: i64,
}

#[derive(Serialize, Debug)]
pub struct UploadedProgram {
    pub program_id: String,
}