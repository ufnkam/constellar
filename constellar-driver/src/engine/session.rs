use std::hash::Hash;

use crate::engine::pool::ConnectionPool;

use super::connection::{Connection, ConnectionParams};

enum SessionState {
    Active,
    Inactive,
}

pub struct Session<'a, C: Connection<P>, P: ConnectionParams + Hash> {
    pool: &'a ConnectionPool<C, P>,
    state: SessionState,
    connection: C,
}

impl<'a, C: Connection<P>, P: ConnectionParams + Hash> Session<'a, C, P> {
    pub fn new(pool: &ConnectionPool<C, P>) -> Self {
        let conn = pool.get_conn();
        return Self {
            pool,
            state: SessionState::Active,
            connection: *conn,
        };
    }

    pub fn close(&self) {
        self.pool.put_conn(self.connection);
        self.state = SessionState::Inactive;
    }
}
