use crate::model::Profile;
use sqlx::Error;
use sqlx::query_as;
use sqlx::PgPool;

pub async fn select_profile(pool: &PgPool, id: i64) -> Result<Option<Profile>, Error> {
    query_as::<_, Profile>(r"
        select * from profile where id = $1
    ")
    .bind(id)
    .fetch_optional(pool)
    .await
}