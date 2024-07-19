pub mod cdriver {
    tonic::include_proto!("cdriver");
}

use super::engine::stash::ConnectionStash;
use crate::driver::Driver;
use crate::engine::connection::{Connection, ConnectionParams};
use crate::server::cdriver::HealthCheckResponse;
use cdriver::c_driver_server::{CDriver, CDriverServer};
use std::env;
use std::hash::Hash;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct CDriverService {}

#[tonic::async_trait]
impl CDriver for CDriverService {
    async fn health_check(
        &self,
        request: Request<()>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        let res = HealthCheckResponse { living: true };
        Ok(Response::new(res))
    }
}

pub struct DriverServer<D: Driver<C, P>, C: Connection<P> + Copy, P: ConnectionParams + Hash + Clone> {
    driver: D,
    stash: Box<ConnectionStash<C, P>>,
}

impl<D: Driver<C, P>, C: Connection<P> + Copy, P: ConnectionParams + Hash + Clone> DriverServer <D, C, P> {
    pub fn new(driver: D) -> Self {
        let stash = driver.make_stash();
        return DriverServer { driver, stash };
    }

    pub async fn run_server(&self) -> Result<(), Box<dyn std::error::Error>> {
        let port = env::var("PORT").unwrap_or("50051".to_string());
        let addr = format!("0.0.0.0:{}", port).parse()?;
        let driver_service = CDriverService::default();
        let driver = CDriverServer::new(driver_service);
        println!("Running on port {}...", port);
        Server::builder().add_service(driver).serve(addr).await?;
        Ok(())
    }
}

