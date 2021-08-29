use crate::{
    context::{Context, DbConnection},
    model::{Id, Todo},
    GraphQLResult, Result,
};
use juniper::graphql_object;
use sqlx::FromRow;

#[derive(Clone, Debug, FromRow)]
pub struct User {
    id: Id<User>,
    name: String,
}

impl User {
    pub async fn create(connection: &mut DbConnection, name: &str) -> Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            r#"
                INSERT INTO users (name)
                VALUES ($1)
                RETURNING
                  id as "id: _",
                  name
            "#,
            name
        )
        .fetch_one(connection)
        .await?)
    }

    pub async fn find(connection: &mut DbConnection, id: Id<User>) -> Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            r#"
                SELECT
                  id as "id: _",
                  name
                FROM users
                WHERE id = $1
            "#,
            id as _
        )
        .fetch_one(connection)
        .await?)
    }
}

#[graphql_object(context = Context)]
impl User {
    fn id(&self) -> juniper::ID {
        self.id.into()
    }

    fn name(&self) -> String {
        self.name.to_string()
    }

    async fn todos(&self, context: &Context) -> GraphQLResult<Vec<Todo>> {
        Ok(Todo::with_user(&mut context.connection().await?, self.id).await?)
    }
}
