use crate::{graphql::types::User, Context};
use juniper::graphql_object;

#[derive(Clone, Copy, Debug)]
pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    async fn users() -> Vec<User> {
        vec![User {
            id: 1,
            name: "User 1".into(),
        }]
    }
}
