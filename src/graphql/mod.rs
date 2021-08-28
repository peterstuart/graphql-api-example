pub mod types;

mod query;

use self::query::Query;
use crate::context::Context;
use juniper::{EmptyMutation, EmptySubscription, RootNode};

type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<Context>::new(),
        EmptySubscription::<Context>::new(),
    )
}
