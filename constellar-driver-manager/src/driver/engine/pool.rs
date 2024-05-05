use std::collections::VecDeque;
use std::hash::Hash;
use std::thread::sleep;
use std::time::Duration;

use crate::driver::engine::access::AccessToken;
use crate::driver::engine::connection::Connection;

use super::connection::ConnectionParams;

pub struct ConnectionPool<C: Connection<P>, P: ConnectionParams + Hash> {
    access_token: AccessToken,
    connections: VecDeque<C>,
    wait_timeout: i32,
}
impl<C: Connection<P>, P: ConnectionParams + Hash> ConnectionPool<C, P> {
    pub fn open(max_size: i32, connection_params: &P, wait_timeout: i32) -> Self {
        let mut connections = VecDeque::new();
        for i in 0..(max_size - 1) {
            let conn = Connection::connect(connection_params);
            connections.push_back(conn);
        }
        let access_token = AccessToken::new(connection_params);
        return ConnectionPool {
            access_token,
            connections,
            wait_timeout,
        };
    }
    pub fn get_conn(&self) -> &C {
        let mut total_sleep_counter = 0;
        while self.connections.is_empty() {
            sleep(Duration::new(1, 0));
            total_sleep_counter += 1;
            if total_sleep_counter >= self.wait_timeout {
                panic!("thread was not able to get connection from pool");
            }
        }
        match self.connections.back() {
            Some(c) => return c,
            None => panic!("connection not found"),
            _ => panic!("wtf"),
        }
    }
    pub fn put_conn(&mut self, connection: C) {
        self.connections.push_back(connection);
    }
    pub fn get_access_token(&self) -> &AccessToken {
        return &self.access_token;
    }
    pub fn verify_access(&self, access_token: &AccessToken) -> bool {
        if !(&self.access_token == access_token) {
            return false;
        }
        return true;
    }
}
