// use std::string;

// use postgres::{Client, NoTls};

use rocket::{fairing::AdHoc, serde::Deserialize, State};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct MyConfig {
    my_rocket_var_1: String,
    my_rocket_var_2: f32,
}

#[macro_use]
extern crate rocket;

#[get("/")]
async fn index() -> &'static str {
    let res = "Home";
    res
}

#[get("/hello")]
async fn hello() -> String {
    let res = format!("Hello {}!", 123);
    res
}

// Note: signature order isn't strict
#[get("/<tweet_id>")]
async fn get_tweet_by_id(tweet_id: String, config: &State<MyConfig>) -> String {
    let res = format!(
        "Tweet {}, {}, {}!",
        tweet_id, config.my_rocket_var_1, config.my_rocket_var_2
    );
    res
}

// #[rocket::main]
// async fn main() -> Result<(), rocket::Error> {
//     let _rocket = rocket::build().mount("/", routes![index]).launch().await?;
//     Ok(())
// }

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, hello, get_tweet_by_id])
        .attach(AdHoc::config::<MyConfig>())
}

#[derive(Debug)]
struct Account {
    id: i32,
    email: String,
    password: String,
}

// fn main() {
//     let mut db = Client::connect(
//         "postgresql://myrustuser:myrustpassword@localhost:15433/myrustdb",
//         NoTls,
//     )
//     .unwrap();

//     let rows = db.query("SELECT * FROM account", &[]);
//     let rows_result = rows.unwrap();
//     for row in rows_result {
//         let account = Account {
//             id: row.get("id"),
//             email: row.get("email"),
//             password: row.get("password"),
//         };

//         println!("Found: {:?}", account);
//     }

//     let db_closed_result = db.close();

//     assert!(db_closed_result.is_ok());
// }
