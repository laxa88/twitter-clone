use rocket::serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Account {
    pub id: i32,
    pub email: String,
    pub password: String,
}
