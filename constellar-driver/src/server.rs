pub mod cdriver {
    tonic::include_proto!("cdriver");
}

use crate::server::cdriver::HealthCheckResponse;
use cdriver::c_driver_server::CDriver;
use std::env;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

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

    //async fn list_data_sources(&self, request: Request<()>) -> Result<Response<Self::ListDataSourcesStream>, Status> {
    //    todo!()
    //}
}

pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let port = env::var("PORT").unwrap_or("50051".to_string());
    let addr = format!("0.0.0.0:{}", port).parse()?;
    let ccore = CDriverService::default();
    let ccore = CDriverService::new(ccore);
    println!("Running on port {}...", port);
    Server::builder().add_service(ccore).serve(addr).await?;
    Ok(())
}
