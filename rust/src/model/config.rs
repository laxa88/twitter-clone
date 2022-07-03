use rocket::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Config {
    pub my_rocket_var_1: String,
    pub my_rocket_var_2: f32,
}
