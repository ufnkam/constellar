use std::hash::Hash;
use crate::engine::Backend;

use crate::engine::pool::ConnectionPool;

use super::connection::{Connection, ConnectionParams};

#[derive(PartialEq)]
enum SessionState {
    Active,
    Inactive,
}

pub struct Session<'a, B: Backend> {
    pool: &'a mut ConnectionPool<B>,
    state: SessionState,
    pub connection: Option<B::Connection>,
}

impl<'a, B: Backend> Session<'a, B> {
    pub fn new(pool: &'a mut ConnectionPool<B>) -> Self {
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

    pub fn set_state(&mut self, state: SessionState) {
        self.state = state;
    }

    pub fn close(&mut self) {
        let conn = self.connection.clone().unwrap_or_else(|| panic!("connection not found"));
        if self.state != SessionState::Active {
            panic!("Session is not active");
        }
        // self.connection.close();
        self.pool.put_conn(conn);
        &self.set_state(SessionState::Inactive);
    }
}
