use crate::engine::Backend;

pub trait ToSql<B: Backend>: Sized {
    fn to_sql(self) -> Result<(Vec<u8>, i32), Box<dyn std::error::Error>>;
}

pub trait FromSql<B: Backend> {
    fn from_sql(raw_value: &mut [u8]) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;
}
