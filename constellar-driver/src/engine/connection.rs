use crate::engine::result::DbResult;
use crate::engine::typing::ToSql;

pub trait ConnectionParams {
    fn uri(&self) -> String;
    fn from_uri(uri: &str) -> Self;
    fn get_backend(&self) -> &'static str;
    fn get_resource(&self) -> &'static str;
    fn get_host(&self) -> &'static str;
}

pub trait Connection<P: ConnectionParams + Clone>: Sized {
    async fn connect(params: P) -> Result<Self, Box<dyn std::error::Error>>;
    async fn execute<R: DbResult + Sized>(&mut self, query: &str) -> Result<R, Box<dyn std::error::Error>>;
    async fn close(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    async fn cancel(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ConnectionStatus {
    Initialized,
    Connected,
    Closed
}
