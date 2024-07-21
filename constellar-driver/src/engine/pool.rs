use std::collections::VecDeque;
use std::hash::Hash;
use std::thread::sleep;
use std::time::Duration;
use crate::engine::Backend;

use crate::engine::connection::Connection;

use super::connection::ConnectionParams;

pub struct ConnectionPool<B: Backend> {
    pub connections: VecDeque<B::Connection>,
    wait_timeout: i32,
    connection_params: B::ConnectionParams,
    max_size: i32,
}
impl<B: Backend> ConnectionPool<B> {
    pub fn new(max_size: i32, wait_timeout: i32, connection_params: B::ConnectionParams) -> Self {
        let mut connections = VecDeque::new();
        return ConnectionPool {
            connections,
            wait_timeout,
            connection_params,
            max_size,
        };
    }

    pub async fn open(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for _ in 0..self.max_size {
            let conn = B::Connection::connect(self.connection_params.clone()).await?;
            self.connections.push_front(conn)
        }
        Ok(())
    }

    pub fn get_conn(&mut self) -> B::Connection {
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

    pub fn put_conn(&mut self, connection: B::Connection) {
        self.connections.push_front(connection);
    }

    pub async fn close(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for mut conn in &mut self.connections {
            conn.close().await?;
        }
        Ok(())
    }
}
