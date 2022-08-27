use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::sqlx::{postgres::PgRow, Row};

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Tweet {
    pub id: i32,
    pub account_id: i32,
    pub body: String,
    pub created_at: DateTime<Utc>,
}

impl Tweet {
    pub fn build_from_db(row: &PgRow) -> Self {
        Self {
            id: row.get(0),
            account_id: row.get(1),
            body: row.get(2),
            created_at: row.get(3),
        }
    }

    pub fn build_from_dbs(rows: &Vec<PgRow>) -> Vec<Self> {
        rows.iter().map(|r| Tweet::build_from_db(r)).collect()
    }
}

#[derive(Debug, Deserialize, FromForm)]
#[serde(crate = "rocket::serde")]
pub struct TweetCreate {
    pub account_id: i32,
    pub body: String,
}
