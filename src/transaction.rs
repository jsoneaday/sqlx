use sqlx::{Error, PgPool};
use sqlx::query_as;

use crate::model::{EntityId, Message};

pub async fn create_profile_and_message(pool: &PgPool, user_name: String, full_name: String, email: String, body: String) -> Result<(EntityId, EntityId), Error> {
    let mut tx = pool.begin().await.unwrap();

    match query_as::<_, EntityId>(r"
        insert into profile
        (user_name, full_name, email)
        values
        ($1, $2, $3)
        returning id
    ")
    .bind(user_name)
    .bind(full_name)
    .bind(email)
    .fetch_one(&mut *tx)
    .await {
        Ok(profile) => {
            match query_as::<_, EntityId>(r"
                insert into message
                (user_id, body)
                values
                ($1, $2)
                returning id
            ")
            .bind(profile.id)
            .bind(body)
            .fetch_one(&mut *tx)
            .await {
                Ok(message) => {
                    _ = tx.commit().await;

                    Ok((profile, message))
                },
                Err(e) => {
                    _ = tx.rollback().await;
                    Err(e)
                }
            }
        },
        Err(e) => {
            _ = tx.rollback().await;
            return Err(e);
        }
    }   
}

pub async fn select_message(pool: &PgPool, id: i64) -> Result<Option<Message>, Error> {
    query_as::<_, Message>(r"
        select * from message where id = $1
    ")
    .bind(id)
    .fetch_optional(pool)
    .await
}