// use crate::engine::{Connection, ConnectionParams, DbResult, ToSqlSerializer};
// use std::hash::Hash;

// pub trait Backend: Sized
// where
//     Self::ConnectionParams: ConnectionParams + Hash + Clone,
//     Self::Connection: Connection<Self>,
//     Self::DbResult: DbResult,
//     Self::ToSqlSerializer: ToSqlSerializer,
// {
//     type ConnectionParams;
//     type Connection;
//     type DbResult;
//     type ToSqlSerializer;
// }
