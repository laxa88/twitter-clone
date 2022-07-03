use rocket::serde::Serialize;
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
    pub fn create_from_db(row: PgRow) -> Self {
        Self {
            id: row.get(0),
            email: row.get(1),
            username: row.get(2),
            password: row.get(3),
        }
    }
}
