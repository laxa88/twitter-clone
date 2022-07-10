use rocket::http::{ContentType, Header};
use rocket::response::status::{Created, NotFound};
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket_db_pools::sqlx;
use rocket_db_pools::sqlx::Row;
use rocket_db_pools::Connection;

use crate::model::account::{Account, AccountCreate};
use crate::model::api::ApiError;
use crate::MyRustDb;

#[get("/db1/<id>")]
pub async fn get_account_by_id_1(mut db: Connection<MyRustDb>, id: i32) -> Option<String> {
    println!("### get 1: {}", id);
    sqlx::query("SELECT * FROM account WHERE id = $1")
        .bind(id)
        .fetch_one(&mut *db)
        .await
        .and_then(|r| {
            let res = Account::build_from_db(r);
            Ok(res.email)
        })
        .ok()
}

#[get("/db2/<id>")]
pub async fn get_account_by_id_2(mut db: Connection<MyRustDb>, id: i32) -> Option<Account> {
    println!("### get 2: {}", id);
    let res = sqlx::query("SELECT * FROM account WHERE id = $1")
        .bind(id)
        .fetch_one(&mut *db)
        .await
        .and_then(|r| {
            let res = Account::build_from_db(r);
            println!("### got {:?}", res);
            Ok(res)
        })
        .ok();

    res
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct MyResponder {
    inner: Account,
    header: ContentType,
    more: Header<'static>,
}

#[get("/db3/<id>")]
pub async fn get_account_by_id_3(mut db: Connection<MyRustDb>, id: i32) -> MyResponder {
    println!("### get 3: {}", id);
    let res = sqlx::query("SELECT * FROM account WHERE id = $1")
        .bind(id)
        .fetch_one(&mut *db)
        .await
        .and_then(|r| {
            let res = Account::build_from_db(r);
            println!("### got {:?}", res);
            Ok(res)
        })
        .ok();

    MyResponder {
        inner: res.unwrap(),
        header: ContentType::new("application", "x-account"),
        more: Header::new("x-header", "header-value"),
    }
}

#[get("/db4/<id>", format = "json")]
pub async fn get_account_by_id_4(mut db: Connection<MyRustDb>, id: i32) -> Json<Option<Account>> {
    println!("### get 4: {}", id);
    let res = sqlx::query("SELECT * FROM account WHERE id = $1")
        .bind(id)
        .fetch_one(&mut *db)
        .await
        .and_then(|r| {
            let res = Account {
                id: r.try_get(0)?,
                email: r.try_get(1)?,
                username: r.try_get(2)?,
                password: r.try_get(3)?,
            };
            println!("### got {:?}", res);
            Ok(res)
        })
        .ok();

    Json(res)
}

#[get("/accounts", format = "json")]
pub async fn list_accounts(mut db: Connection<MyRustDb>) -> Json<Vec<Account>> {
    sqlx::query("SELECT * FROM account")
        .fetch_all(&mut *db)
        .await
        .map(|row| {
            let res = row
                .iter()
                .map(|r| {
                    let acc = Account {
                        id: r.try_get(0).unwrap(),
                        email: r.try_get(1).unwrap(),
                        username: r.try_get(2).unwrap(),
                        password: r.try_get(3).unwrap(),
                    };

                    acc
                })
                .collect();

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
        .map(|r| {
            let acc = Account {
                id: r.try_get(0).unwrap(),
                email: r.try_get(1).unwrap(),
                username: r.try_get(2).unwrap(),
                password: r.try_get(3).unwrap(),
            };

            Json(acc)
        })
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
