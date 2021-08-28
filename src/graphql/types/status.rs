use juniper::GraphQLEnum;

#[derive(Clone, Copy, Debug, GraphQLEnum)]
pub enum Status {
    Incomplete,
    Complete,
}
