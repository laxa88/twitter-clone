use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::sqlx::{postgres::PgRow, Row};

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Account {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
}

impl Account {
    pub fn build_from_db(row: PgRow) -> Self {
        Self {
            id: row.get(0),
            email: row.get(1),
            username: row.get(2),
            password: row.get(3),
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
