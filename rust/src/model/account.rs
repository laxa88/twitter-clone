use rocket::http::ContentType;
use rocket::response::Responder;
use rocket::serde::{Deserialize, Serialize};
use rocket::Response;
use rocket_db_pools::sqlx::{postgres::PgRow, Row};

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
