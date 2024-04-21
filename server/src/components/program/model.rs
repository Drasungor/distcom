use serde_derive::{Serialize, Deserialize};

use crate::utils::jwt_helpers::GeneratedToken;

// Controller input models

#[derive(Deserialize)]
pub struct UploadProgram {
    // Amount of seconds that will be waited before considering a requested program-input duo as abandoned
    pub execution_timeout: i64,
}

// // Useful models

// #[derive(Serialize)]
// pub struct LoginTokens {
//     pub basic_token: GeneratedToken,
//     pub refresh_token: GeneratedToken,
// }

// // Controller output models

// #[derive(Serialize)]
// pub struct InitSession {
//     pub token: String,
//     pub refresh_token: String,
// }