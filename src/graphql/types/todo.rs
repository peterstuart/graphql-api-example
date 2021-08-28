use crate::graphql::types::Status;
use juniper::GraphQLObject;

#[derive(Clone, Debug, GraphQLObject)]
pub struct Todo {
    pub id: i32,
    pub name: String,
    pub status: Status,
}
