use std::collections::VecDeque;
use std::hash::Hash;
use std::thread::sleep;
use std::time::Duration;

use crate::engine::connection::Connection;

use super::connection::ConnectionParams;

pub struct ConnectionPool<C: Connection<P>, P: ConnectionParams + Hash> {
    connections: VecDeque<C>,
    wait_timeout: i32,
    connection_params: P,
    max_size: i32,
}
impl<C: Connection<P>, P: ConnectionParams + Hash> ConnectionPool<C, P> {
    pub fn new(max_size: i32, wait_timeout: i32, connection_params: P) -> Self {
        let mut connections = VecDeque::new();
        return ConnectionPool {
            connections,
            wait_timeout,
            connection_params,
            max_size,
        };
    }

    pub fn open(&mut self) {
        for _ in 0..(self.max_size - 1) {
            let conn = Connection::connect(&self.connection_params);
            self.connections.push_back(conn);
        }
    }

    pub fn get_conn(&mut self) -> C {
        let mut total_sleep_counter = 0;
        while self.connections.is_empty() {
            sleep(Duration::new(1, 0));
            total_sleep_counter += 1;
            if total_sleep_counter >= self.wait_timeout {
                panic!("thread was not able to get connection from pool");
            }
        }
        match self.connections.pop_back() {
            Some(c) => return c,
            None => panic!("connection not found"),
            _ => panic!("wtf"),
        }
    }

    pub fn put_conn(&mut self, connection: C) {
        self.connections.push_back(connection);
    }
}
