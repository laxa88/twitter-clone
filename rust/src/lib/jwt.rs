use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rocket::serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::model::account::Account;

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Claims {
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

pub fn sign_jwt(account: Account) -> String {
    let start = SystemTime::now();
    let since_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    // Create and return signed token
    let claims = Claims {
        exp: since_epoch.as_secs() as usize,
        id: account.id,
        email: account.email,
        username: account.username,
    };

    match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(b"some-secret-key"),
    ) {
        Ok(t) => t,
        Err(_) => panic!(), // TODO: handle accordingly
    }
}

pub fn validate_jwt(jwt: &str) -> Result<Account, jsonwebtoken::errors::Error> {
    match decode::<Claims>(
        &jwt,
        &DecodingKey::from_secret(b"some-secret-key"),
        &Validation::default(),
    ) {
        Ok(r) => Ok(Account {
            id: r.claims.id,
            email: r.claims.email,
            username: r.claims.username,
        }),
        Err(e) => Err(e),
    }
}
