use crate::{
    engine::{AccessToken, ConnectionParams},
    grpc::cdriver::DataSourceInfo,
};

use super::{Connection, Query};

pub struct DriverNativeDataSource {
    pub host: String,
    pub resource: String,
    pub access_token: AccessToken,
    pub name: String,
    connection: Connection,
}

impl DriverNativeDataSource {
    pub fn new(
        connection_params: ConnectionParams,
        name: Option<&str>,
    ) -> Result<(Self, AccessToken), Box<dyn std::error::Error>> {
        let access_token = AccessToken::new(&connection_params)?;

        let source_name = match name {
            Some(name) => name.to_string(),
            None => format!("{}@{}", connection_params.host, connection_params.database),
        };
        let host = connection_params.host.clone();
        let resource = connection_params.database.clone();

        let connection = Connection::new(connection_params)?;

        Ok((
            Self {
                host,
                resource,
                access_token,
                name: source_name,
                connection,
            },
            access_token,
        ))
    }

    pub fn connect(&mut self) {
        todo!()
    }

    pub fn verify_access(&self, access_token: &AccessToken) -> bool {
        &self.access_token == access_token
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn execute_command(&self, query: &Query) -> Result<(), Box<dyn std::error::Error>> {
        let _ = self.connection.execute(query);
        Ok(())
    }
}

impl Into<DataSourceInfo> for DriverNativeDataSource {
    fn into(self) -> DataSourceInfo {
        DataSourceInfo {
            name: self.get_name(),
            host: self.host.to_string(),
            port: "5432".to_string(),
            user: "xd".to_string(),
        }
    }
}

impl Into<DataSourceInfo> for &DriverNativeDataSource {
    fn into(self) -> DataSourceInfo {
        DataSourceInfo {
            name: self.get_name(),
            host: self.host.to_string(),
            port: "5432".to_string(),
            user: "xd".to_string(),
        }
    }
}
