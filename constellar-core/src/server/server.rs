pub mod ccore {
    tonic::include_proto!("ccore");
}

use ccore::c_core_server::{CCore, CCoreServer};
use std::env;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use self::ccore::{HealthCheckRequest, HealthCheckResponse};

#[derive(Debug, Default)]
pub struct CCoreService {}

#[tonic::async_trait]
impl CCore for CCoreService {
    async fn health_check(
        &self,
        request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        print!("Health check requested.");
        let reply = HealthCheckResponse { living: true };
        Ok(Response::new(reply))
    }
}

pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let port = env::var("PORT").unwrap_or("50051".to_string());
    let addr = format!("0.0.0.0:{}", port).parse()?;
    let ccore = CCoreService::default();
    let ccore = CCoreServer::new(ccore);
    println!("Running on port {}...", port);
    Server::builder().add_service(ccore).serve(addr).await?;
    Ok(())
}
