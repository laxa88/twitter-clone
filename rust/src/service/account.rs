use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket_db_pools::sqlx;
use rocket_db_pools::Connection;

use crate::model::account::{Account, AccountCreate, AccountLogin};
use crate::model::api::ApiError;
use crate::MyRustDb;

use crate::lib::jwt::{sign_jwt, Claims};

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

#[post("/account/create", data = "<new_account>")]
pub async fn create_account(
    new_account: Json<AccountCreate>,
    mut db: Connection<MyRustDb>,
) -> Result<Created<Json<String>>, Json<ApiError>> {
    sqlx::query("INSERT INTO account (email, username, password) VALUES ($1, $2, $3)")
        .bind(new_account.email.clone())
        .bind(new_account.username.clone())
        .bind(new_account.password.clone())
        .execute(&mut *db)
        .await
        .map(|_| Created::new("/").body(Json("Ok".to_string())))
        .map_err(|e| {
            Json(ApiError {
                details: e.to_string(),
            })
        })
}

#[post("/account/login", data = "<login>")]
pub async fn login_account(
    login: Json<AccountLogin>,
    mut db: Connection<MyRustDb>,
) -> Result<String, NotFound<Json<ApiError>>> {
    // get user input

    // validate user input

    // get and validate user against db
    let jwt = sqlx::query("SELECT * FROM account WHERE username = $1 AND password = $2")
        .bind(login.username.clone())
        .bind(login.password.clone())
        .fetch_one(&mut *db)
        .await
        .map(|r| {
            // if user found, login is successful.
            let account = Account::build_from_db(&r);

            // Create and return signed token
            let my_claims = Claims {
                exp: 10000,
                email: account.email,
                username: account.username,
            };

            sign_jwt(&my_claims)
        })
        .map_err(|e| {
            NotFound(Json(ApiError {
                details: e.to_string(),
            }))
        });

    jwt
}
