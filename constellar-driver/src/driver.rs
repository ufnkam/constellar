use std::hash::Hash;
use crate::engine::connection::{Connection, ConnectionParams};
use crate::engine::stash::ConnectionStash;
use crate::sql::Column;


pub trait Driver <C: Connection<P> + Copy, P: ConnectionParams + Hash> {
    fn get_name(self) -> String;

    fn make_stash(&self) -> Box<ConnectionStash<C, P>> {
        return Box::new(ConnectionStash::new());
    }
}
