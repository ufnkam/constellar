use std::hash::Hash;

use crate::engine::pool::ConnectionPool;

use super::connection::{Connection, ConnectionParams};

#[derive(PartialEq)]
enum SessionState {
    Active,
    Inactive,
}

pub struct Session<'a, C: Connection<P> + Copy, P: ConnectionParams + Hash> {
    pool: &'a mut ConnectionPool<C, P>,
    state: SessionState,
    pub connection: Option<C>,
}

impl<'a, C: Connection<P> + Copy, P: ConnectionParams + Hash> Session<'a, C, P> {
    pub fn new(pool: &'a mut ConnectionPool<C, P>) -> Self {
        return Self {
            pool: pool,
            state: SessionState::Inactive,
            connection: None,
        };
    }

    pub fn begin(&mut self) {
        self.connection = Some(self.pool.get_conn());
        self.state = SessionState::Active;
    }

    pub fn close(&mut self) {
        // self.connection.close();
        if self.state == SessionState::Active && self.connection.is_some() {
            let conn = self.connection.unwrap();
            self.pool.put_conn(conn);
            self.state = SessionState::Inactive;
        }
    }
}
