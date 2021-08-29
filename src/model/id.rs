use std::marker::PhantomData;

use sqlx::{
    encode::IsNull,
    error::BoxDynError,
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef},
    Decode, Encode, Postgres,
};

#[derive(Debug, Eq, PartialEq)]
pub struct Id<T> {
    value: i32,
    phantom: PhantomData<T>,
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Id {
            value: self.value,
            phantom: PhantomData,
        }
    }
}

impl<T> Copy for Id<T> {}

impl<T> sqlx::Type<Postgres> for Id<T> {
    fn type_info() -> PgTypeInfo {
        i32::type_info()
    }
}

impl<T> Encode<'_, Postgres> for Id<T> {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
        i32::encode_by_ref(&self.value, buf)
    }
}

impl<T> Decode<'_, Postgres> for Id<T> {
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        let value = i32::decode(value)?;
        Ok(Self {
            value,
            phantom: PhantomData,
        })
    }
}

impl<T> From<juniper::ID> for Id<T> {
    fn from(id: juniper::ID) -> Self {
        Self {
            value: (*id).parse().unwrap(),
            phantom: PhantomData,
        }
    }
}

impl<T> From<Id<T>> for juniper::ID {
    fn from(id: Id<T>) -> Self {
        juniper::ID::new(id.value.to_string())
    }
}
