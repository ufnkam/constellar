use super::access::AccessToken;
use super::connection::ConnectionParams;
use super::data_source::DriverNativeDataSource;
use std::io::ErrorKind;

pub struct ConnectionStash {
    pub stash: Vec<DriverNativeDataSource>,
}

impl ConnectionStash {
    pub fn new() -> Self {
        let stash = Vec::new();
        ConnectionStash { stash }
    }

    pub fn get_data_source(
        &self,
        name: &str,
        access_token: &AccessToken,
    ) -> Result<&DriverNativeDataSource, std::io::Error> {
        for dnds in self.stash.iter() {
            if dnds.name == name {
                return Ok(dnds);
            }
        }
        Err(std::io::Error::new(
            ErrorKind::InvalidInput,
            "data source not found",
        ))
    }

    pub fn create_data_source(
        &mut self,
        params: ConnectionParams,
        name: Option<&'static str>,
    ) -> Result<(AccessToken, String), Box<dyn std::error::Error>> {
        let (dnds, token) = DriverNativeDataSource::new(params, name)?;
        let name = dnds.get_name();
        self.stash.push(dnds);
        Ok((token, name))
    }

    pub fn connect_data_source(&mut self) {}
}
