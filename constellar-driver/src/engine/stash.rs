use std::hash::Hash;

use super::{
    access::AccessToken,
    connection::{Connection, ConnectionParams},
    data_source::DataSource,
    pool::ConnectionPool,
    session::Session,
};

struct ConnectionStash<C: Connection<P>, P: ConnectionParams + Hash> {
    stash: Vec<ConnectionPool<C, P>>,
}

impl<C: Connection<P>, P: ConnectionParams + Hash> ConnectionStash<C, P> {
    fn new() -> Self {
        let stash = Vec::new();
        return ConnectionStash { stash };
    }
    fn make_pool(
        &mut self,
        connection_params: P,
        max_size: i32,
        wait_timeout: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut pool = ConnectionPool::new(max_size, connection_params, wait_timeout);
        let already_exists = match self.find_pool(pool.get_access_token()) {
            Some(_) => true,
            None => false,
        };

        if already_exists {
            panic!("Connection already exists");
        }
        pool.open();
        self.stash.push(pool);
        Ok(())
    }
    fn make_session<'a>(
        &self,
        access_token: &AccessToken,
    ) -> Result<Session<C, P>, Box<dyn std::error::Error>> {
        if self.stash.len() == 0 {
            panic!("connetion stash is empty")
        }

        let found_pool = self.find_pool(access_token);
        match found_pool {
            Some(pool) => Ok(Session::new(pool)),
            None => {
                panic!("pool not found");
            }
        }
    }

    fn list_data_sources(&self) -> Vec<&DataSource> {
        let mut ds = Vec::new();
        for i in &self.stash {
            ds.push(&i.data_source);
        }
        return ds;
    }

    fn find_pool(&self, access_token: &AccessToken) -> Option<&ConnectionPool<C, P>> {
        for pool in self.stash.iter() {
            if pool.verify_access(access_token) {
                return Some(pool);
            }
        }
        return None;
    }
}
