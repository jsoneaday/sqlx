use sqlx::{postgres::PgQueryResult, PgPool};



pub async fn update_profile(pool: &PgPool, id: i64, user_name: String) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query::<sqlx::postgres::Postgres>(r"
        update profile
        set user_name = $1
        where id = $2
    ")
    .bind(user_name)
    .bind(id)
    .execute(pool)
    .await
}