use rocket::http::Status;
use rocket::response::status::{Created, Custom, Unauthorized};
use rocket::serde::json::Json;
use rocket_db_pools::sqlx;
use rocket_db_pools::Connection;

use crate::model::account::{Account, AccountCreate, AccountLogin};
use crate::model::api::ApiError;
use crate::MyRustDb;

use crate::lib::jwt::sign_jwt;

#[post("/signup", data = "<new_account>")]
pub async fn signup(
    new_account: Json<AccountCreate>,
    mut db: Connection<MyRustDb>,
) -> Result<Created<Json<String>>, Custom<Json<ApiError>>> {
    sqlx::query("INSERT INTO account (email, username, password) VALUES ($1, $2, $3)")
        .bind(new_account.email.clone())
        .bind(new_account.username.clone())
        .bind(new_account.password.clone())
        .execute(&mut *db)
        .await
        .map(|_| Created::new("/").body(Json("Ok".to_string())))
        .map_err(|e| {
            // FIXME return status code doesn't work
            Custom(
                Status::InternalServerError,
                Json(ApiError {
                    details: e.to_string(),
                }),
            )
        })
}

#[post("/login", data = "<login>")]
pub async fn login(
    login: Json<AccountLogin>,
    mut db: Connection<MyRustDb>,
) -> Result<String, Unauthorized<Json<ApiError>>> {
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

            sign_jwt(account)
        })
        .map_err(|_| {
            Unauthorized(Some(Json(ApiError {
                details: "Incorrect username or password".to_string(),
            })))
        });

    jwt
}
