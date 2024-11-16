use sqlx::PgPool;
use sqlx::Error;
use sqlx::query_as;
use crate::model::EntityId;


pub async fn insert_profile(pool: &PgPool, user_name: String, full_name: String, email: String) -> Result<EntityId, Error> {
    query_as::<_, EntityId>(r"
        insert into profile
        (user_name, full_name, email)
        values
        ($1, $2, $3)
        returning id
    ")
    .bind(user_name)
    .bind(full_name)
    .bind(email)
    .fetch_one(pool)
    .await
}