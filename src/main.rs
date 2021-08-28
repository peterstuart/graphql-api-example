mod context;
mod graphql;
mod model;
mod types;

pub use types::{GraphQLResult, Result};

use context::Context;
use std::env;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;

    env_logger::init();
    let log = warp::log("graphql-api-example");

    let port = env::var("API_PORT")?.parse()?;

    let context = Context::new().await?;

    let context_filter = warp::any().map(move || context.clone());
    let graphql_filter =
        juniper_warp::make_graphql_filter(graphql::schema(), context_filter.boxed());

    warp::serve(
        warp::get()
            .and(warp::path("graphiql"))
            .and(juniper_warp::graphiql_filter("/graphql", None))
            .or(warp::path("graphql").and(graphql_filter))
            .with(log),
    )
    .run(([127, 0, 0, 1], port))
    .await;

    Ok(())
}
