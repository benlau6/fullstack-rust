use crate::configuration::get_configuration;
use chrono::{Duration, Utc};
use jsonwebtoken::{errors::Result, DecodingKey, EncodingKey, Header, TokenData, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Admin,
    User,
}

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Claims {
    // aud: String, // Optional. Audience
    exp: i64, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: i64, // Optional. Issued at (as UTC timestamp)
    // iss: String, // Optional. Issuer
    // nbf: usize, // Optional. Not Before (as UTC timestamp)
    pub sub: uuid::Uuid, // Optional. Subject (whom token refers to)
    pub role: Role,
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User ID: {}", self.sub)
    }
}

impl Claims {
    pub fn new(sub: uuid::Uuid, role: Role) -> Self {
        let now = Utc::now();
        Self {
            exp: (now + Duration::hours(1)).timestamp(),
            iat: now.timestamp(),
            sub,
            role,
        }
    }
}

pub fn encode(claims: &Claims) -> Result<String> {
    jsonwebtoken::encode(&Header::default(), claims, &KEYS.encoding)
}

pub fn decode(token: &str) -> Result<TokenData<Claims>> {
    jsonwebtoken::decode::<Claims>(token, &KEYS.decoding, &Validation::default())
}

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let configuration = get_configuration().expect("Failed to load configuration.");
    let secret = configuration.security.secret_key;
    Keys::new(secret.as_bytes())
});

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_jwt_encoding_and_decoding() {
        let my_claims = Claims::new(uuid::Uuid::new_v4(), Role::User);
        let token = encode(&my_claims).unwrap();
        let claims = decode(&token).unwrap().claims;
        assert_eq!(my_claims, claims)
    }
}
