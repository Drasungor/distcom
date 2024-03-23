use serde_derive::{Serialize, Deserialize};

use crate::utils::jwt_helpers::GeneratedToken;

// // Controller input models

// #[derive(Deserialize)]
// pub struct Credentials {
//     pub username: String,
//     pub password: String,
// }

// #[derive(Deserialize)]
// pub struct ReceivedNewAccount {
//     pub username: String,
//     pub password: String,
//     pub name: String,
//     pub description: String,
// }

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