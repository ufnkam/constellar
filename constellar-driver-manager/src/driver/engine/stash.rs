use std::hash::Hash;

use super::{
    access::AccessToken,
    connection::{Connection, ConnectionParams},
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
        connection_params: &P,
        max_size: i32,
        wait_timeout: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let pool = ConnectionPool::open(max_size, connection_params, wait_timeout);
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

        let mut found_pool = None;
        for pool in self.stash.iter() {
            if pool.verify_access(access_token) {
                found_pool = Some(pool);
                break;
            }
        }

        match found_pool {
            Some(pool) => Ok(Session::new(pool)),
            None => {
                panic!("pool not found");
            }
        }
    }
}
