use std::hash::Hash;

use crate::driver::engine::pool::ConnectionPool;

use super::connection::{Connection, ConnectionParams};

pub struct Session<'a, C: Connection, P: ConnectionParams + Hash> {
    pool: &'a ConnectionPool<C, P>,
}

impl<'a, C: Connection, P: ConnectionParams + Hash> Session<'a, C, P> {
    pub fn new(pool: &ConnectionPool<C, P>) -> Self {}
}
