use crate::engine::{FromSql, ToSql};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use libpq_sys::Oid;
use crate::drivers::postgres::PgBackend;

#[derive(Eq, PartialEq)]
pub enum PgType {
    Int4,
}

impl PgType {
    pub fn match_from_oid(type_oid: Oid) -> PgType {
        match type_oid {
            23 => PgType::Int4,
            _ => panic!("Type not supported"),
        }
    }
}

impl FromSql<PgBackend> for i32 {
    fn from_sql(raw_value: &mut [u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut raw_cp = raw_value.to_vec();
        let result = ReadBytesExt::read_i32::<BigEndian>(&mut raw_cp.iter().as_slice())?;
        Ok(result)
    }
}

impl ToSql<PgBackend> for i32 {
    fn to_sql(self) -> Result<(Vec<u8>, i32), Box<dyn std::error::Error>> {
        Ok((self.to_be_bytes().to_vec(), 23))
    }
}

#[cfg(test)]
mod test {
    use crate::engine::FromSql;

    #[test]
    fn from_sql_int() -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = [0, 0, 0, 1];
        let x: i32 = FromSql::from_sql(&mut buffer)?;
        assert_eq!(x, 1);
        Ok(())
    }
}
