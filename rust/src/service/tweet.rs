use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket_db_pools::sqlx;
use rocket_db_pools::Connection;

use crate::model::api::ApiError;
use crate::model::tweet::{Tweet, TweetCreate};
use crate::MyRustDb;

use crate::model::account::Account;

#[get("/tweet/<tweet_id>")]
pub async fn get_tweet_by_id(
    mut db: Connection<MyRustDb>,
    _account: Account, // FromRequest validation
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

#[get("/tweets/<username>")]
pub async fn get_tweets_by_username(
    mut db: Connection<MyRustDb>,
    _account: Account, // FromRequest validation
    username: String,
) -> Result<Json<Vec<Tweet>>, NotFound<Json<ApiError>>> {
    sqlx::query(
        "SELECT t.* FROM tweet t JOIN account a ON t.account_id = a.id WHERE a.username = $1 ORDER BY created_at DESC",
    )
    .bind(username)
    .fetch_all(&mut *db)
    .await
    .map(|r| Json(Tweet::build_from_dbs(&r)))
    .map_err(|e| {
        NotFound(Json(ApiError {
            details: e.to_string(),
        }))
    })
}

#[post("/tweet/create", data = "<new_tweet>")]
pub async fn create_tweet(
    new_tweet: Json<TweetCreate>,
    _account: Account, // FromRequest validation
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
