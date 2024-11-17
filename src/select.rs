use sqlx::PgPool;

use crate::model::Profile;

pub async fn select_profile(pool: &PgPool, id: i64) -> Result<Option<Profile>, sqlx::Error> {
    sqlx::query_as::<_, Profile> (r"
        select * from profile where id = $1
    ")
    .bind(id)
    .fetch_optional(pool)
    .await
}