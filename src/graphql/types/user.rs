use crate::{
    context::Context,
    graphql::types::{Status, Todo},
};
use juniper::graphql_object;

#[derive(Clone, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[graphql_object(context = Context)]
impl User {
    fn id(&self) -> i32 {
        self.id
    }

    fn name(&self) -> String {
        self.name.to_string()
    }

    fn todos(&self) -> Vec<Todo> {
        vec![Todo {
            id: 1,
            name: "Todo 1".into(),
            status: Status::Incomplete,
        }]
    }
}
