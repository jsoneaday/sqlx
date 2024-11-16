use insert::insert_profile;
use select::select_profile;
use sqlx::postgres::PgPoolOptions;

pub mod model;
pub mod select;
pub mod update;
pub mod insert;
pub mod transaction;

#[tokio::main]
async fn main() {
    let pg_url = "postgres://sqlx:sqlx@localhost:5433/sqlx";
    let pool = PgPoolOptions::new().max_connections(4).connect(pg_url).await.unwrap();

    // let result = insert_profile(&pool, "dave".to_string(), "Dave Choi".to_string(), "dave@test.com".to_string()).await.unwrap();
    let result = select_profile(&pool, 1).await.unwrap().unwrap();
    println!("result: {:?}", result);
}
