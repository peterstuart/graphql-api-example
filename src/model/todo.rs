use crate::{
    context::{Context, DbConnection},
    model::{Id, Status, User},
    GraphQLResult, Result,
};
use juniper::graphql_object;
use sqlx::FromRow;

#[derive(Clone, Debug, FromRow)]
pub struct Todo {
    id: Id<Todo>,
    user_id: Id<User>,
    name: String,
    status: Status,
}

impl Todo {
    pub async fn create(
        connection: &mut DbConnection,
        user_id: Id<User>,
        name: &str,
        status: Status,
    ) -> Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            r#"
                INSERT INTO todos (user_id, name, status)
                VALUES ($1, $2, $3)
                RETURNING id as "id: _", user_id as "user_id: _", name, status as "status: _"
            "#,
            user_id as _,
            name,
            status as _
        )
        .fetch_one(connection)
        .await?)
    }

    pub async fn with_user(connection: &mut DbConnection, user_id: Id<User>) -> Result<Vec<Self>> {
        Ok(sqlx::query_as!(
            Self,
            r#"
                SELECT id as "id: _", user_id as "user_id: _", name, status as "status: _"
                FROM todos
                WHERE user_id = $1
                ORDER BY id
            "#,
            user_id as _
        )
        .fetch_all(connection)
        .await?)
    }
}

#[graphql_object(context = Context)]
impl Todo {
    fn id(&self) -> juniper::ID {
        self.id.into()
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
