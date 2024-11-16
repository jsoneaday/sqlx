use delete::delete_profile;
use insert::insert_profile;
use select::select_profile;
use sqlx::postgres::PgPoolOptions;
use transaction::{create_profile_and_message, select_message};
use update::update_profile;

pub mod model;
pub mod select;
pub mod update;
pub mod insert;
pub mod delete;
pub mod transaction;

#[tokio::main]
async fn main() {
    let pg_url = "postgres://sqlx:sqlx@localhost:5433/sqlx";
    let pool = PgPoolOptions::new().max_connections(4).connect(pg_url).await.unwrap();

    // let result = insert_profile(&pool, "dave".to_string(), "Dave Choi".to_string(), "dave@test.com".to_string()).await.unwrap();
    // let result = select_profile(&pool, 1).await.unwrap().unwrap();
    // let result = update_profile(&pool, 1, "james".to_string()).await.unwrap();
    // let result = delete_profile(&pool, 1).await.unwrap();

    let result = create_profile_and_message(&pool, "linda".to_string(), "Linda Lynn".to_string(), "linda@test.com".to_string(), "hello world".to_string()).await.unwrap();
    let profile = select_profile(&pool, result.0.id).await.unwrap();
    let message = select_message(&pool, result.1.id).await.unwrap();
    println!("profile: {:?}, message: {:?}", profile, message);
}
