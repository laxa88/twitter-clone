use rocket::http::{ContentType, Header};
use rocket::response::Responder;
use rocket::serde::Serialize;
use rocket::Response;
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

#[get("/db1/<id>")]
async fn get_account_by_id_1(mut db: Connection<MyRustDb>, id: i32) -> Option<String> {
    println!("### get 1: {}", id);
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

#[get("/db2/<id>")]
async fn get_account_by_id_2(mut db: Connection<MyRustDb>, id: i32) -> Option<Account> {
    println!("### get 2: {}", id);
    let res = sqlx::query("SELECT * FROM account WHERE id = $1")
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
            Ok(res)
        })
        .ok();

    res
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
struct MyResponder {
    inner: Account,
    header: ContentType,
    more: Header<'static>,
}

#[get("/db3/<id>")]
async fn get_account_by_id_3(mut db: Connection<MyRustDb>, id: i32) -> MyResponder {
    println!("### get 3: {}", id);
    let res = sqlx::query("SELECT * FROM account WHERE id = $1")
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
            Ok(res)
        })
        .ok();

    MyResponder {
        inner: res.unwrap(),
        header: ContentType::new("application", "x-account"),
        more: Header::new("x-header", "header-value"),
    }
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
                get_tweet_by_id,
                get_account_by_id_1,
                get_account_by_id_2,
                get_account_by_id_3
            ],
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
