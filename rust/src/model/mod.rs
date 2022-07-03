use rocket_db_pools::sqlx::{self};
use rocket_db_pools::Database;

#[derive(Database)]
#[database("myrustdb")]
pub struct MyRustDb(sqlx::PgPool);
