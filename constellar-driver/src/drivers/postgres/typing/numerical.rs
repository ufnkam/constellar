use std::error::Error;
use arrow::datatypes::ToByteSlice;
use byteorder::{BigEndian, NetworkEndian, ReadBytesExt};
use bytes::BytesMut;
use crate::drivers::postgres::PgBackend;
use crate::engine::{FromSql, ToSql};

impl FromSql<PgBackend> for i32 {
    fn from_sql(raw_value: &mut [u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut raw_cp = raw_value;
        let result = ReadBytesExt::read_i32::<NetworkEndian>(&mut raw_cp.iter().as_slice())?;
        Ok(result)
    }
}

impl FromSql<PgBackend> for i64 {
    fn from_sql(raw_value: &mut [u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut raw_cp = raw_value;
        let result = ReadBytesExt::read_i64::<NetworkEndian>(&mut raw_cp.iter().as_slice())?;
        Ok(result)
    }

}

impl FromSql<PgBackend> for f32 {
    fn from_sql(raw_value: &mut [u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut raw_cp = raw_value;
        let result = ReadBytesExt::read_f32::<BigEndian>(&mut raw_cp.iter().as_slice())?;
        Ok(result)
    }
}

impl FromSql<PgBackend> for f64 {
    fn from_sql(raw_value: &mut [u8]) -> Result<Self, Box<dyn Error>> where Self: Sized {
        let mut raw_cp = raw_value;
        let result = ReadBytesExt::read_f64::<BigEndian>(&mut raw_cp.iter().as_slice())?;
        Ok(result)
    }
}

impl ToSql<PgBackend> for i32 {
    fn to_sql(&self) -> (BytesMut, i32) {
        (BytesMut::from(self.to_be_bytes().to_byte_slice()), 23)
    }
}

impl ToSql<PgBackend> for i64 {
    fn to_sql(&self) -> (BytesMut, i32) {
        (BytesMut::from(self.to_be_bytes().to_byte_slice()), 20)

    }
}


impl ToSql<PgBackend> for i16 {
    fn to_sql(&self) -> (BytesMut, i32) {
        (BytesMut::from(self.to_byte_slice().to_byte_slice()), 21)
    }
}


impl ToSql<PgBackend> for f32 {
    fn to_sql(&self) -> (BytesMut, i32) {
        (BytesMut::from(self.to_be_bytes().to_byte_slice()), 700)
    }
}

impl ToSql<PgBackend> for f64 {
    fn to_sql(&self) -> (BytesMut, i32) {
        (BytesMut::from(self.to_be_bytes().to_byte_slice()), 701)
    }
}

impl ToSql<PgBackend> for u32 {
    fn to_sql(&self) -> (BytesMut, i32) {
        (BytesMut::from(self.to_be_bytes().to_byte_slice()), 23)
    }
}

impl ToSql<PgBackend> for u64 {
    fn to_sql(&self) -> (BytesMut, i32) {
        (BytesMut::from(self.to_be_bytes().to_byte_slice()), 20)
    }
}



#[cfg(test)]
mod test {
    use crate::engine::{FromSql, ToSql};

    #[test]
    fn to_sql_int() -> Result<(), Box<dyn std::error::Error>> {
        let x = 1;
        let (mut buffer, oid) = x.to_sql();
        assert_eq!(oid, 23);
        let to_assert: i32 = FromSql::from_sql(&mut buffer)?;
        assert_eq!(to_assert, x);
        Ok(())
    }

    #[test]
    fn to_sql_float() -> Result<(), Box<dyn std::error::Error>> {
        let x = 1.23;
        let (mut buffer, oid) = x.to_sql();
        assert_eq!(oid, 701);
        let to_assert: f64 = FromSql::from_sql(&mut buffer)?;
        assert_eq!(to_assert, x);
        Ok(())
    }

}
