use bytes::BytesMut;
use crate::engine::Backend;

pub trait ToSql<B: Backend> {
    fn to_sql(&self) -> (BytesMut, i32);
}

pub trait FromSql<B: Backend>: Sized {
    fn from_sql(raw_value: &mut [u8]) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;
}
