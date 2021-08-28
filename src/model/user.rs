use crate::{
    context::{Context, DbConnection},
    model::Todo,
    GraphQLResult, Result,
};
use juniper::graphql_object;
use sqlx::FromRow;

#[derive(Clone, Debug, FromRow)]
pub struct User {
    id: i32,
    name: String,
}

impl User {
    pub async fn create(connection: &mut DbConnection, name: &str) -> Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            "INSERT INTO users (name) VALUES ($1) RETURNING *",
            name
        )
        .fetch_one(connection)
        .await?)
    }

    pub async fn find(connection: &mut DbConnection, id: i32) -> Result<Self> {
        Ok(
            sqlx::query_as!(Self, "SELECT * FROM users WHERE id = $1", id)
                .fetch_one(connection)
                .await?,
        )
    }
}

#[graphql_object(context = Context)]
impl User {
    fn id(&self) -> i32 {
        self.id
    }

    fn name(&self) -> String {
        self.name.to_string()
    }

    async fn todos(&self, context: &Context) -> GraphQLResult<Vec<Todo>> {
        Ok(Todo::with_user(&mut context.connection().await?, self.id).await?)
    }
}
