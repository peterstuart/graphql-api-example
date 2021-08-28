use juniper::GraphQLEnum;

#[derive(Clone, Copy, Debug, GraphQLEnum, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
pub enum Status {
    Incomplete,
    Complete,
}
