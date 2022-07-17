use rocket::http::{ContentType, Status};
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::response::Responder;
use rocket::serde::{Deserialize, Serialize};
use rocket::Response;
use rocket_db_pools::sqlx::{postgres::PgRow, Row};

use crate::lib::jwt::validate_jwt;

use super::api::ApiError;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Account {
    pub id: i32,
    pub email: String,
    pub username: String,
}

impl Account {
    pub fn build_from_db(row: &PgRow) -> Self {
        Self {
            id: row.get(0),
            email: row.get(1),
            username: row.get(2),
        }
    }
}

#[derive(Debug, Deserialize, FromForm)]
#[serde(crate = "rocket::serde")]
pub struct AccountCreate {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, FromForm)]
#[serde(crate = "rocket::serde")]
pub struct AccountLogin {
    pub username: String,
    pub password: String,
}

impl<'r> Responder<'r, 'static> for Account {
    fn respond_to(self, req: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let string = format!("{}: {}", self.id, self.email);

        Response::build_from(string.respond_to(req)?)
            .raw_header("X-Account-Id", self.id.to_string())
            .raw_header("X-Account-Email", self.email)
            .header(ContentType::new("application", "x-account"))
            .ok()
    }
}

// Reference: https://stackoverflow.com/questions/69271458/lifetimes-do-not-match-method-in-trait-when-get-token-from-request
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Account {
    type Error = ApiError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let auth: Vec<_> = request.headers().get("authorization").collect();
        let jwt_str = auth.get(0).unwrap();
        let jwt = &jwt_str[7..]; // Omit "Bearer " prefix

        match validate_jwt(jwt) {
            Ok(account) => Outcome::Success(account),
            Err(e) => Outcome::Failure((
                Status::BadRequest,
                ApiError {
                    details: e.to_string(),
                },
            )),
        }
    }
}
