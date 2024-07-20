use std::hash::Hash;

use super::{Backend, connection::{Connection, ConnectionParams}, pool::ConnectionPool, session::Session};
use crate::engine::access::AccessToken;

pub struct DriverNativeDataSource<B: Backend> {
    host: &'static str,
    resource: &'static str,
    pool: ConnectionPool<B>,
    access_token: AccessToken,
    name: &'static str,
}

impl<B: Backend> DriverNativeDataSource<B> {
    pub fn new(
        connection_params: B::ConnectionParams,
        name: Option<&'static str>,
        max_size: i32,
        wait_timeout: i32,
    ) -> Self {
        let host = connection_params.get_host();
        let backend = connection_params.get_backend();
        let access_token = AccessToken::new(&connection_params);
        let name = match name {
            Some(n) => n,
            None => host,
        };
        let pool = ConnectionPool::new(max_size, wait_timeout, connection_params);
        return Self {
            pool,
            host,
            resource: backend,
            access_token,
            name,
        };
    }

    pub fn connect(&mut self) {
        self.pool.open();
    }

    pub fn obtain_session(&mut self) -> Session<B> {
        let pool = &mut self.pool;
        return Session::new(pool);
    }

    pub fn verify_access(&self, access_token: &AccessToken) -> bool {
        return self.access_token == *access_token;
    }
}
