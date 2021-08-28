mod mutation;
mod query;

use self::{mutation::Mutation, query::Query};
use crate::context::Context;
use juniper::{EmptySubscription, RootNode};

type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::<Context>::new())
}
