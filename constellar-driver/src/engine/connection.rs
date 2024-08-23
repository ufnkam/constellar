use crate::engine::Query;
use arrow_odbc::odbc_api::{
    Connection as OdbcConnection, ConnectionOptions, Environment, ResultSetMetadata,
};
use arrow_odbc::OdbcReaderBuilder;
use std::hash::{Hash, Hasher};
use std::task::Wake;
use tonic::transport::server::Connected;

pub struct ConnectionParams {
    pub driver: &'static str,
    pub host: &'static str,
    pub port: &'static u32,
    pub user: &'static str,
    pub password: &'static str,
    pub database: &'static str,
    pub application_name: Option<&'static str>,
}

impl ConnectionParams {
    pub fn new(
        driver: &'static str,
        host: &'static str,
        port: &'static u32,
        user: &'static str,
        password: &'static str,
        database: &'static str,
        application_name: Option<&'static str>,
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
        let query = Query {
            query: "select now();",
            params: (),
        };
        let start_time = std::time::Instant::now();
        let _ = self.execute(&query)?;
        let finish_time = std::time::Instant::now() - start_time;
        Ok(finish_time.as_millis())
    }

    pub fn execute(
        &mut self,
        query: &Query,
    ) -> Result<Vec<arrow::array::RecordBatch>, Box<dyn std::error::Error>> {
        let mut conn = self.odbc_environment.connect_with_connection_string(
            &self.params.into_odbc_string(),
            ConnectionOptions::default(),
        )?;
        self.status = ConnectionStatus::Connected;
        let cursor = conn.execute(query.query, query.params)?.unwrap();

        let arrays = OdbcReaderBuilder::new().build(cursor)?;
        let mut batches = Vec::new();
        for array in arrays {
            batches.push(array?);
        }

        std::mem::drop(conn);
        self.status = ConnectionStatus::Closed;
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
            "PostgreSQL",
            "localhost",
            &9999,
            "postgres_user",
            "postgres_password",
            "postgres",
            None,
        );

        let mut connection = Connection::new(params)?;
        let query = Query {
            query: "select 1 as some, 2 as elsesome;",
            // query: "SELECT * FROM shopping where client_id = ? and channel = ?;",
            params: (),
        };
        let res = connection.execute(&query)?;

        Ok(())
    }

    #[test]
    fn test_ping() -> Result<(), Box<dyn std::error::Error>> {
        let params = ConnectionParams::new(
            "PostgreSQL",
            "localhost",
            &9999,
            "postgres_user",
            "postgres_password",
            "postgres",
            None,
        );

        let mut connection = Connection::new(params)?;
        let latency = connection.ping()?;
        println!("Pong time: {}ms", latency);
        Ok(())
    }
}
