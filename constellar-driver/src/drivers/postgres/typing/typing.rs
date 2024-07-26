use libpq_sys::Oid;

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

