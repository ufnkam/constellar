use crate::engine::Query;
use crate::grpc::cdriver::ConnectionInfo;
use arrow_odbc::odbc_api::{Connection as OdbcConnection, ConnectionOptions, Environment};
use arrow_odbc::OdbcReaderBuilder;
use std::hash::{Hash, Hasher};

pub struct ConnectionParams {
    pub driver: String,
    pub host: String,
    pub port: u64,
    pub user: String,
    pub password: String,
    pub database: String,
    pub application_name: Option<String>,
}

impl ConnectionParams {
    pub fn new(
        driver: String,
        host: String,
        port: u64,
        user: String,
        password: String,
        database: String,
        application_name: Option<String>,
    ) -> Self {
        ConnectionParams {
            driver,
            host,
            port,
            user,
            password,
            database,
            application_name,
        }
    }
    pub fn into_odbc_string(&self) -> String {
        let template = format!(
            "
            Driver={driver};\
            Server={host};\
            UID={user};\
            PWD={password};\
            PORT={port};\
            DATABASE={database};\
        ",
            driver = self.driver,
            host = self.host,
            user = self.user,
            password = self.password,
            port = self.port,
            database = self.database
        );
        template
    }
}

impl Hash for ConnectionParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.database.hash(state);
        self.host.hash(state);
        self.user.hash(state);
    }
}

impl From<ConnectionInfo> for ConnectionParams {
    fn from(value: ConnectionInfo) -> Self {
        ConnectionParams {
            driver: value.driver,
            host: value.host,
            port: value.port,
            user: value.user,
            password: value.password,
            database: value.database,
            application_name: None,
        }
    }
}

impl From<&ConnectionInfo> for ConnectionParams {
    fn from(value: &ConnectionInfo) -> Self {
        ConnectionParams {
            driver: value.driver.clone(),
            host: value.host.clone(),
            port: value.port,
            user: value.user.clone(),
            password: value.password.clone(),
            database: value.database.clone(),
            application_name: None,
        }
    }
}

pub struct Connection {
    params: ConnectionParams,
    odbc_environment: Environment,
    status: ConnectionStatus,
}

impl Connection {
    pub fn new(params: ConnectionParams) -> Result<Connection, Box<dyn std::error::Error>> {
        Ok(Connection {
            odbc_environment: Environment::new()?,
            params,
            status: ConnectionStatus::Initialized,
        })
    }

    pub fn ping(&mut self) -> Result<u128, Box<dyn std::error::Error>> {
        let query = Query::new("select now();".to_string(), ());
        let start_time = std::time::Instant::now();
        let _ = self.execute(&query)?;
        let finish_time = std::time::Instant::now() - start_time;
        Ok(finish_time.as_millis())
    }

    pub fn execute(
        &self,
        query: &Query,
    ) -> Result<Vec<arrow::array::RecordBatch>, Box<dyn std::error::Error>> {
        println!("{:?}", &query.query);
        let conn = self.odbc_environment.connect_with_connection_string(
            &self.params.into_odbc_string(),
            ConnectionOptions::default(),
        )?;
        // self.status = ConnectionStatus::Connected;
        let cursor = conn.execute(&query.query, query.params)?.unwrap();

        let arrays = OdbcReaderBuilder::new().build(cursor)?;
        let mut batches = Vec::new();
        for array in arrays {
            batches.push(array?);
        }

        std::mem::drop(conn);

        println!("{:?}", batches.len());
        // self.status = ConnectionStatus::Closed;
        Ok(batches)
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ConnectionStatus {
    Initialized,
    Connected,
    Closed,
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrow_odbc::odbc_api::IntoParameter;

    #[test]
    fn test_connection() -> Result<(), Box<dyn std::error::Error>> {
        let params = ConnectionParams::new(
            "PostgreSQL".to_string(),
            "localhost".to_string(),
            9999,
            "postgres_user".to_string(),
            "postgres_password".to_string(),
            "postgres".to_string(),
            None,
        );

        let mut connection = Connection::new(params)?;
        let query = Query::new("select 1 as some, 2 as elsesome".to_string(), ());
        let res = connection.execute(&query)?;
        assert_eq!(res[0].num_rows(), 1);
        Ok(())
    }

    #[test]
    fn test_ping() -> Result<(), Box<dyn std::error::Error>> {
        let params = ConnectionParams::new(
            "PostgreSQL".to_string(),
            "localhost".to_string(),
            "9999".parse().expect("not a number"),
            "postgres_user".to_string(),
            "postgres_password".to_string(),
            "postgres".to_string(),
            None,
        );

        let mut connection = Connection::new(params)?;
        let latency = connection.ping()?;
        println!("Pong time: {}ms", latency);
        Ok(())
    }
}
