mod context;
mod graphql;

use context::Context;
use warp::Filter;

#[tokio::main]
async fn main() {
    env_logger::init();
    let log = warp::log("graphql-api-example");

    let state = warp::any().map(|| Context);
    let graphql_filter = juniper_warp::make_graphql_filter(graphql::schema(), state.boxed());

    warp::serve(
        warp::get()
            .and(warp::path("graphiql"))
            .and(juniper_warp::graphiql_filter("/graphql", None))
            .or(warp::path("graphql").and(graphql_filter))
            .with(log),
    )
    .run(([127, 0, 0, 1], 8080))
    .await
}
