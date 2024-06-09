use std::hash::Hash;

use super::access::AccessToken;
use super::connection::{Connection, ConnectionParams};
use super::data_source::DriverNativeDataSource;

pub struct ConnectionStash<C: Connection<P> + Copy, P: ConnectionParams + Hash> {
    stash: Vec<DriverNativeDataSource<P, C>>,
}

impl<C: Connection<P> + Copy, P: ConnectionParams + Hash> ConnectionStash<C, P> {
    pub fn new() -> Self {
        let stash = Vec::new();
        return ConnectionStash { stash };
    }

    pub fn get_data_source(&self, access_token: &AccessToken) -> Option<&DriverNativeDataSource<P, C>> {
        for dnds in self.stash.iter() {
            if dnds.verify_access(access_token) {
                return Some(dnds);
            }
        }
        return None;
    }

    pub fn add_driver_data_source(&mut self, dnds: DriverNativeDataSource<P, C>) {
        self.stash.push(dnds);
    }
}
