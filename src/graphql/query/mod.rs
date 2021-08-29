use crate::{model::User, Context};
use juniper::{graphql_object, FieldError, ID};

#[derive(Clone, Copy, Debug)]
pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    async fn user(context: &Context, id: ID) -> Result<Vec<User>, FieldError> {
        let mut connection = context.connection().await?;

        Ok(vec![User::find(&mut connection, id.into()).await?])
    }
}
