use jsonwebtoken::{encode, decode, Header, Validation, Algorithm, EncodingKey, DecodingKey};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub exp: usize,
    pub organization_id: String,
    pub token_id: String,
}

#[derive(Serialize)]
pub struct GeneratedToken {
    pub token_id: String,
    pub token: String,
}

pub fn generate_jwt(secret: &str, organization_id: &str, expiration_secs: u64) -> GeneratedToken {
    let token_id = Uuid::new_v4().to_string();
    let expiration_time = SystemTime::now().checked_add(std::time::Duration::from_secs(expiration_secs))
        .expect("SystemTime overflow");

    let exp = expiration_time.duration_since(UNIX_EPOCH)
        .expect("Time went backwards").as_secs() as usize;

    let claims = Claims {
        organization_id: organization_id.to_owned(),
        exp,
        token_id: token_id.clone(),
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
                              .expect("Failed to create token");

    return GeneratedToken {
        token_id,
        token,
    };
}

pub fn validate_jwt(secret: &str, token: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);
    let decoded = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &validation)?;

    // Check if token has expired
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("error in epochs line").as_secs() as usize;
    if decoded.claims.exp < current_time {
        return Err(jsonwebtoken::errors::ErrorKind::ExpiredSignature.into());
    }
    return Ok(decoded.claims.organization_id);
}
