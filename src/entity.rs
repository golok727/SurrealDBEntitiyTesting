use futures_util::future::LocalBoxFuture;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

pub trait BaseEntity<Entity>
where
    Entity: Serialize + for<'de> Deserialize<'de>,
{
    const TABLE_NAME: &'static str;

    fn create(
        &self,
        db: impl Deref<Target = Surreal<Client>> + 'static,
        data: impl Serialize + 'static,
    ) -> LocalBoxFuture<'static, Result<Option<Entity>, surrealdb::Error>> {
        Box::pin(async move {
            let query = "CREATE type::table($table) CONTENT $data RETURN *;";

            let mut response = db
                .query(query)
                .bind(("data", data))
                .bind(("table", Self::TABLE_NAME.clone()))
                .await?;

            let created_entity: Option<Entity> = response.take(0)?;

            Ok(created_entity)
        })
    }

    fn get_all(
        db: impl Deref<Target = Surreal<Client>> + 'static,
    ) -> LocalBoxFuture<'static, Result<Vec<Entity>, surrealdb::Error>> {
        Box::pin(async move {
            let query = "SELECT * from type::table($table);";

            let mut response = db
                .query(query)
                .bind(("table", Self::TABLE_NAME.clone()))
                .await?;

            let entities = response.take::<Vec<Entity>>(0)?;

            Ok(entities)
        })
    }
}
