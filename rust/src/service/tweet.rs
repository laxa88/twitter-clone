use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket_db_pools::sqlx;
use rocket_db_pools::Connection;

use crate::model::api::ApiError;
use crate::model::tweet::{Tweet, TweetCreate};
use crate::MyRustDb;

#[get("/tweet/<tweet_id>")]
pub async fn get_tweet_by_id(
    mut db: Connection<MyRustDb>,
    tweet_id: i32,
) -> Result<Json<Tweet>, NotFound<Json<ApiError>>> {
    sqlx::query("SELECT * FROM tweet WHERE id = $1")
        .bind(tweet_id)
        .fetch_one(&mut *db)
        .await
        .map(|r| Json(Tweet::build_from_db(&r)))
        .map_err(|e| {
            NotFound(Json(ApiError {
                details: e.to_string(),
            }))
        })
}

// TODO get tweets by user

#[post("/tweet/create", data = "<new_tweet>")]
pub async fn create_tweet(
    new_tweet: Json<TweetCreate>,
    mut db: Connection<MyRustDb>,
) -> Result<Created<Json<String>>, Json<ApiError>> {
    sqlx::query("INSERT INTO tweet (account_id, body) VALUES ($1, $2)")
        .bind(new_tweet.account_id.clone())
        .bind(new_tweet.body.clone())
        .execute(&mut *db)
        .await
        .map(|_| Created::new("/").body(Json("Ok".to_string())))
        .map_err(|e| {
            Json(ApiError {
                details: e.to_string(),
            })
        })
}
