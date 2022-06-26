// use std::string;

// use postgres::{Client, NoTls};

use rocket::serde::Serialize;
use rocket::{fairing::AdHoc, serde::Deserialize, State};
use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::{Connection, Database};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct MyConfig {
    my_rocket_var_1: String,
    my_rocket_var_2: f32,
}

#[derive(Database)]
#[database("myrustdb")]
struct MyRustDb(sqlx::PgPool);

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

#[get("/db/<id>")]
async fn get_account_by_id_1(mut db: Connection<MyRustDb>, id: i32) -> Option<String> {
    println!("### get: {}", id);
    sqlx::query("SELECT * FROM account WHERE id = $1")
        .bind(id)
        .fetch_one(&mut *db)
        .await
        .and_then(|r| {
            let res = Account {
                id: r.try_get(0)?,
                email: r.try_get(1)?,
                password: r.try_get(2)?,
            };
            println!("### got {:?}", res);
            Ok(res.email)
        })
        .ok()
}

// #[rocket::main]
// async fn main() -> Result<(), rocket::Error> {
//     let _rocket = rocket::build().mount("/", routes![index]).launch().await?;
//     Ok(())
// }

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(MyRustDb::init())
        .mount(
            "/",
            routes![index, hello, get_tweet_by_id, get_account_by_id_1],
        )
        .attach(AdHoc::config::<MyConfig>())
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
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
