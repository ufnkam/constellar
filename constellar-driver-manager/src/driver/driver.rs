use crate::driver::sql::Column;
use std::collections::HashMap;

pub trait SqlDriver {
    fn get_table_schema<T: Column>(self, table: &'static str) -> Vec<T>;
}
pub trait NoSqlDriver {}

pub trait ConnectionParams {
    fn uri(&self) -> String;
    fn from_uri(self, uri: &str) -> Self;
}

pub trait DriverConnection {
    fn new() -> Self;
    fn connect<T: ConnectionParams>(
        &mut self,
        connection_params: T,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait Driver {
    fn get_name(self) -> String;
}

pub trait DbCommand {
    fn connect(self);
    fn execute<K, V>(self, statement: &str, exec_opts: HashMap<K, V>);
    fn executemany<K, V>(self, statement: &str, exec_opts: HashMap<K, V>);
}
