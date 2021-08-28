use crate::{
    context::{Context, DbConnection},
    model::{Status, User},
    GraphQLResult, Result,
};
use juniper::graphql_object;
use sqlx::FromRow;

#[derive(Clone, Debug, FromRow)]
pub struct Todo {
    id: i32,
    user_id: i32,
    name: String,
    status: Status,
}

impl Todo {
    pub async fn create(
        connection: &mut DbConnection,
        user_id: i32,
        name: &str,
        status: Status,
    ) -> Result<Self> {
        Ok(sqlx::query_as::<_, Self>(
            "INSERT INTO todos (user_id, name, status) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(user_id)
        .bind(name)
        .bind(status)
        .fetch_one(connection)
        .await?)
    }

    pub async fn with_user(connection: &mut DbConnection, user_id: i32) -> Result<Vec<Self>> {
        Ok(
            sqlx::query_as::<_, Self>("SELECT * FROM todos WHERE user_id = $1 ORDER BY id")
                .bind(user_id)
                .fetch_all(connection)
                .await?,
        )
    }
}

#[graphql_object(context = Context)]
impl Todo {
    fn id(&self) -> i32 {
        self.id
    }

    async fn user(&self, context: &Context) -> GraphQLResult<User> {
        Ok(User::find(&mut context.connection().await?, self.user_id).await?)
    }

    fn name(&self) -> String {
        self.name.to_string()
    }

    fn status(&self) -> Status {
        self.status
    }
}
