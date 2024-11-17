use delete::delete_profile;
use insert::insert_profile;
use select::select_profile;
use sqlx::postgres::PgPoolOptions;
use update::update_profile;

pub mod model;
pub mod select;
pub mod update;
pub mod insert;
pub mod delete;

#[tokio::main]
async fn main() {
    let pg_url = "postgres://sqlx:sqlx@localhost:5433/sqlx";
    let pool = PgPoolOptions::new().max_connections(4).connect(pg_url).await.unwrap();

    let result = delete_profile(&pool, 1).await.unwrap();
    let result = select_profile(&pool, 1).await.unwrap();
    println!("result: {:?}", result);
}
