use std::hash::Hash;
use crate::driver::Driver;
use crate::engine::connection::{Connection, ConnectionParams};
use crate::server::DriverServer;

pub struct DriverBuilder<D, P, C>
    where D: Driver<C, P>,
          P: ConnectionParams + Hash,
          C: Connection<P> + Copy
{
    server: DriverServer<D, C, P>,
}

impl<D, C, P> DriverBuilder<D, P, C>
where D: Driver<C, P>,
        P: ConnectionParams + Hash,
        C: Connection<P> + Copy
{

    pub fn new(driver: D) -> Self {
        let server = DriverServer::new(driver);
        DriverBuilder {
            server,
        }
    }
}