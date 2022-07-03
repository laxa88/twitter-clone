use rocket::State;

use crate::model::config::Config;

// Note: signature order isn't strict
#[get("/<tweet_id>")]
pub async fn get_tweet_by_id(tweet_id: String, config: &State<Config>) -> String {
    let res = format!(
        "Tweet {}, {}, {}!",
        tweet_id, config.my_rocket_var_1, config.my_rocket_var_2
    );
    res
}
