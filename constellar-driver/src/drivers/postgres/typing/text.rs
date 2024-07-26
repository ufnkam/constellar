use crate::drivers::postgres::PgBackend;
use crate::engine::{FromSql, ToSql};
use byteorder::{BigEndian, ReadBytesExt};
use std::error::Error;
use bytes::BytesMut;


impl FromSql<PgBackend> for String {
    fn from_sql(raw_value: &mut [u8]) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized,
    {
        Ok(std::str::from_utf8(raw_value)?.to_string())
    }
}

impl ToSql<PgBackend> for &'static str {
    fn to_sql(&self) -> (BytesMut, i32) {
        (BytesMut::from(self.as_bytes()), 25)
    }
}

impl ToSql<PgBackend> for String {
    fn to_sql(&self) -> (BytesMut, i32) {
        (BytesMut::from(self.as_bytes()), 25)
    }
}

impl ToSql<PgBackend> for str {
    fn to_sql(&self) -> (BytesMut, i32) {
        (BytesMut::from(self.as_bytes()), 25)
    }
}

#[cfg(test)]
mod tests {
    use std::str;
    use crate::engine::{FromSql, ToSql};

    #[test]
    fn to_sql_string() -> Result<(), Box<dyn std::error::Error>> {
        let s = "Hello, World!".to_string();
        let (mut bytes, oid) = s.clone().to_sql();
        assert_eq!(oid, 25);
        let to_assert = String::from_sql(&mut bytes)?;
        assert_eq!(to_assert, s);
        Ok(())
    }

    #[test]
    fn to_sql_str() -> Result<(), Box<dyn std::error::Error>> {
        let s = "Hello, World!";
        let (mut bytes, oid) = s.clone().to_sql();
        assert_eq!(oid, 25);
        let to_assert: String = FromSql::from_sql(&mut bytes)?;
        assert_eq!(to_assert, s);
        Ok(())
    }
}
