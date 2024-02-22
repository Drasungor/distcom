use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

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