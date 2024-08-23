use crate::engine::{AccessToken, ConnectionParams};
use std::hash::Hash;

pub struct DriverNativeDataSource {
    host: &'static str,
    resource: &'static str,
    access_token: AccessToken,
    name: String,
}

impl DriverNativeDataSource {
    pub fn new(
        connection_params: ConnectionParams,
        name: Option<&str>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let access_token = AccessToken::new(&connection_params)?;

        let source_name = match name {
            Some(name) => name.to_string(),
            None => format!("{}@{}", connection_params.host, connection_params.database)
        };

        Ok(Self {
            host: connection_params.host,
            resource: connection_params.database,
            access_token,
            name: source_name,
        })
    }

    pub fn connect(&mut self) {
        todo!()
    }

    pub fn verify_access(&self, access_token: &AccessToken) -> bool {
        &self.access_token == access_token
    }
}
