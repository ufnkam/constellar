use std::hash::Hash;

use super::access::AccessToken;
use super::connection::{ConnectionParams};
use super::data_source::DriverNativeDataSource;

pub struct ConnectionStash {
    stash: Vec<DriverNativeDataSource>,
}

impl ConnectionStash {
    pub fn new() -> Self {
        let stash = Vec::new();
        ConnectionStash { stash }
    }

    pub fn get_data_source(&self, access_token: &AccessToken) -> Option<&DriverNativeDataSource> {
        for dnds in self.stash.iter() {
            if dnds.verify_access(access_token) {
                return Some(dnds);
            }
        }
        None
    }

    pub fn create_data_source(
        &mut self,
        params: ConnectionParams,
        name: Option<&'static str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let dnds = DriverNativeDataSource::new(params, name)?;
        self.stash.push(dnds);
        Ok(())
    }

    pub fn connect_data_source(&mut self) {}
}
