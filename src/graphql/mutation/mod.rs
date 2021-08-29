use crate::{
    model::{Status, Todo, User},
    Context, GraphQLResult,
};
use juniper::{graphql_object, ID};

#[derive(Clone, Copy, Debug)]
pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    async fn create_user(context: &Context, name: String) -> GraphQLResult<Vec<User>> {
        let mut connection = context.connection().await?;

        Ok(vec![User::create(&mut connection, &name).await?])
    }

    async fn create_todo(
        context: &Context,
        user_id: ID,
        name: String,
        status: Status,
    ) -> GraphQLResult<Todo> {
        let mut connection = context.connection().await?;

        Ok(Todo::create(&mut connection, user_id.into(), &name, status).await?)
    }
}
