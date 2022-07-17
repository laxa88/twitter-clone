use jsonwebtoken::{encode, EncodingKey, Header};
use rocket::serde::{Deserialize, Serialize};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    // aud: String, // Optional. Audience
    pub exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    // iat: usize, // Optional. Issued at (as UTC timestamp)
    // iss: String, // Optional. Issuer
    // nbf: usize, // Optional. Not Before (as UTC timestamp)
    // sub: String, // Optional. Subject (whom token refers to)
    pub id: i32,
    pub email: String,
    pub username: String,
}

pub fn sign_jwt(claims: &Claims) -> String {
    match encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(b"some-secret-key"),
    ) {
        Ok(t) => t,
        Err(_) => panic!(), // TODO: handle accordingly
    }
}
