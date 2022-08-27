use model::config::Config;
use rocket::fairing::AdHoc;
use rocket_db_pools::Database;

mod service;
use service::account::get_account_by_id;
use service::account::list_accounts;
use service::auth::login;
use service::auth::signup;
use service::cors::all_options;
use service::tweet::create_tweet;
use service::tweet::get_tweet_by_id;
use service::tweet::get_tweets_by_username;

mod lib;
use lib::cors::CORS;

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
        .attach(CORS)
        .mount(
            "/",
            routes![
                all_options,
                signup,
                create_tweet,
                get_account_by_id,
                get_tweet_by_id,
                get_tweets_by_username,
                index,
                list_accounts,
                login,
            ],
        )
        .attach(AdHoc::config::<Config>())
}
