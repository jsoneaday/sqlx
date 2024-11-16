use sqlx::postgres::PgQueryResult;

pub async fn delete_profile(pool: &sqlx::PgPool, id: i64) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query::<sqlx::postgres::Postgres>(r"
        delete from profile where id = $1
    ")
    .bind(id)
    .execute(pool)
    .await
}