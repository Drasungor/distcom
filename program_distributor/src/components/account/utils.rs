use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

use crate::utils::jwt_helpers::{generate_jwt, GeneratedToken};
use crate::common::config::CONFIG_OBJECT;

use super::model::LoginTokens;

pub fn generate_password_hash(password: String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt).expect("Error in password hash generation").to_string();
    return password_hash;
}

pub fn is_password_valid(password: String, password_hash: String) -> bool {
    let parsed_hash = PasswordHash::new(&password_hash).expect("Error in password hash object generation");
    return Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok();
}

pub fn generate_basic_token(organization_id: &str) -> GeneratedToken {
    return generate_jwt(&CONFIG_OBJECT.token.basic_token_secret, organization_id, 
        &CONFIG_OBJECT.token.basic_token_minutes_duration * 60);
}

pub fn generate_refresh_token(organization_id: &str) -> GeneratedToken {
    return generate_jwt(&CONFIG_OBJECT.token.refresh_token_secret, organization_id, 
        &CONFIG_OBJECT.token.refresh_token_days_duration * 24 * 60 * 60);
}

pub fn generate_login_tokens(organization_id: &str) -> LoginTokens {
    return LoginTokens {
        basic_token: generate_basic_token(organization_id),
        refresh_token: generate_refresh_token(organization_id),
    };
}