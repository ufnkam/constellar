use crate::drivers::postgres::{PgConnection, PgConnectionParams, PgResultWrapper};
use crate::engine::{Backend, ToSql};

pub struct PgBackend {}

impl Backend for PgBackend {
    type ConnectionParams = PgConnectionParams;
    type Connection = PgConnection;
    type DbResult = PgResultWrapper;
}
