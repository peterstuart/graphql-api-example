use juniper::FieldError;

pub type Result<T> = anyhow::Result<T>;
pub type GraphQLResult<T> = std::result::Result<T, FieldError>;
