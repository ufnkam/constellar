use crate::engine::typing::ToSql;
use crate::engine::Backend;

pub trait ConnectionParams {
    fn uri(&self) -> String;
    fn from_uri(uri: &str) -> Self;
    fn get_backend(&self) -> &'static str;
    fn get_resource(&self) -> &'static str;
    fn get_host(&self) -> &'static str;
}

pub trait Connection<B: Backend>: Sized {
    async fn connect(params: B::ConnectionParams) -> Result<Self, Box<dyn std::error::Error>>;
    async fn execute(
        &mut self,
        query: &str,
        params: &[&(dyn ToSql<B>)],
    ) -> Result<B::DbResult, Box<dyn std::error::Error>>;
    async fn close(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    async fn cancel(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ConnectionStatus {
    Initialized,
    Connected,
    Closed,
}
