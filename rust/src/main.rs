use model::config::Config;
use rocket::fairing::AdHoc;
use rocket_db_pools::Database;

mod service;
use service::account::create_account;
use service::account::get_account_by_id;
use service::account::list_accounts;
use service::tweet::get_tweet_by_id;

mod model;
use model::api::MyRustDb;

#[macro_use]
extern crate rocket;

#[get("/")]
async fn index() -> &'static str {
    let res = "Server is running";
    res
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(MyRustDb::init())
        .mount(
            "/",
            routes![
                index,
                get_account_by_id,
                list_accounts,
                create_account,
                get_tweet_by_id,
            ],
        )
        .attach(AdHoc::config::<Config>())
}
