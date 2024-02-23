use jsonwebtoken::{encode, decode, Header, Validation, Algorithm, EncodingKey, DecodingKey};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

fn generate_jwt(secret: &str, subject: &str, expiration_secs: u64) -> String {
    let expiration_time = SystemTime::now().checked_add(std::time::Duration::from_secs(expiration_secs))
        .expect("SystemTime overflow");

    let exp = expiration_time.duration_since(UNIX_EPOCH)
        .expect("Time went backwards").as_secs() as usize;

    let claims = Claims {
        sub: subject.to_owned(),
        exp,
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
        .expect("Failed to create token");

    token
}

fn validate_jwt(secret: &str, token: &str) -> Result<(), jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);
    let decoded = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &validation)?;

    // Check if token has expired
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as usize;
    if decoded.claims.exp < current_time {
        return Err(jsonwebtoken::errors::ErrorKind::ExpiredSignature.into());
    }

    Ok(())
}

fn main() {
    let secret = "your_secret_key_here";
    let subject = "user_id_or_any_other_subject";
    let expiration_secs = 3600; // 1 hour

    let token = generate_jwt(secret, subject, expiration_secs);
    println!("Generated JWT token: {}", token);

    match validate_jwt(secret, &token) {
        Ok(_) => println!("JWT token is valid"),
        Err(e) => println!("JWT token validation failed: {}", e),
    }
}
