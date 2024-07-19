use std::hash::Hash;

use super::access::AccessToken;
use super::connection::{Connection, ConnectionParams};
use super::data_source::DriverNativeDataSource;

pub struct ConnectionStash<C: Connection<P> + Copy, P: ConnectionParams + Hash + Clone> {
    stash: Vec<DriverNativeDataSource<P, C>>,
}

impl<C: Connection<P> + Copy, P: ConnectionParams + Hash + Clone> ConnectionStash<C, P> {
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

    pub fn create_data_source(&mut self, params: P, wait_timeout: i32, max_size: i32, name: Option<&'static str>) {
        let dnds = DriverNativeDataSource::new(params, name, max_size, wait_timeout);
        self.stash.push(dnds);
    }

    pub fn connect_data_source(&mut self) {}
}
