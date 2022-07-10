use model::config::Config;
use rocket::fairing::AdHoc;
use rocket::http::ContentType;
use rocket::response::Responder;
use rocket::Response;
use rocket_db_pools::Database;

mod service;
use service::account::create_account;
use service::account::get_account_by_id_1;
use service::account::get_account_by_id_2;
use service::account::get_account_by_id_3;
use service::account::get_account_by_id_4;
use service::tweet::get_tweet_by_id;

mod model;
use model::account::Account;
use model::api::MyRustDb;

#[macro_use]
extern crate rocket;

#[get("/")]
async fn index() -> &'static str {
    let res = "Server is running";
    res
}

#[get("/hello")]
async fn hello() -> String {
    let res = format!("Hello {}!", 123);
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
                hello,
                create_account,
                get_tweet_by_id,
                get_account_by_id_1,
                get_account_by_id_2,
                get_account_by_id_3,
                get_account_by_id_4
            ],
        )
        .attach(AdHoc::config::<Config>())
}

impl<'r> Responder<'r, 'static> for Account {
    fn respond_to(self, req: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let string = format!("{}: {}, {}", self.id, self.email, self.password);

        Response::build_from(string.respond_to(req)?)
            .raw_header("X-Account-Id", self.id.to_string())
            .raw_header("X-Account-Email", self.email)
            .header(ContentType::new("application", "x-account"))
            .ok()
    }
}
