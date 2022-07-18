use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket_db_pools::sqlx;
use rocket_db_pools::Connection;

use crate::model::account::Account;
use crate::model::api::ApiError;
use crate::MyRustDb;

#[get("/accounts", format = "json")]
pub async fn list_accounts(mut db: Connection<MyRustDb>) -> Json<Vec<Account>> {
    sqlx::query("SELECT * FROM account")
        .fetch_all(&mut *db)
        .await
        .map(|row| {
            let res = row.iter().map(|r| Account::build_from_db(r)).collect();

            Json(res)
        })
        .expect("Oh no")
}

#[get("/account/<id>", format = "json")]
pub async fn get_account_by_id(
    mut db: Connection<MyRustDb>,
    id: i32,
) -> Result<Json<Account>, NotFound<Json<ApiError>>> {
    sqlx::query("SELECT * FROM account WHERE id = $1")
        .bind(id)
        .fetch_one(&mut *db)
        .await
        .map(|r| Json(Account::build_from_db(&r)))
        .map_err(|e| {
            NotFound(Json(ApiError {
                details: e.to_string(),
            }))
        })
}
