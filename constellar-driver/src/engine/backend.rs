use std::hash::Hash;
use crate::engine::{Connection, ConnectionParams, DbResult};

pub trait Backend: Sized
where
    Self::ConnectionParams: ConnectionParams + Hash + Clone,
    Self::Connection: Connection<Self> + Clone,
    Self::DbResult: DbResult
{
    type ConnectionParams;
    type Connection;
    type DbResult;
}