use sqlx::{postgres::PgQueryResult, PgPool};


pub async fn delete_profile(pool: &PgPool, id: i64) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query::<_>(r"
        delete from profile
        where id = $1
    ")
    .bind(id)
    .execute(pool)
    .await
}