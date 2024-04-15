pub mod cdriver_manager {
    tonic::include_proto!("cdrivermanager");
}

use cdriver_manager::driver_manager_server::{DriverManager, DriverManagerServer};
use futures_util::FutureExt;
use tokio::sync::oneshot;
use tonic::{transport::Server, Request, Response, Status};

use self::cdriver_manager::{HealthCheckRequest, HealthCheckResponse};

pub struct DriverManagerRuntime {
    port: i32,
    running: bool,
    sender: oneshot::Sender<()>,
    receiver: oneshot::Receiver<()>,
}

#[derive(Default)]
pub struct DriverManagerService {}

#[tonic::async_trait]
impl DriverManager for DriverManagerService {
    async fn health_check(
        &self,
        request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        print!("Health check requested.");
        let reply = HealthCheckResponse { living: true };
        Ok(Response::new(reply))
    }
}

impl DriverManagerRuntime {
    pub fn default() -> Self {
        let (tx, rx) = oneshot::channel::<()>();
        DriverManagerRuntime {
            port: 50052,
            running: false,
            sender: tx,
            receiver: rx,
        }
    }
    pub fn new(port: i32) -> Self {
        let (tx, rx) = oneshot::channel::<()>();
        DriverManagerRuntime {
            port,
            running: false,
            sender: tx,
            receiver: rx,
        }
    }
    pub fn start(self) {
        self.start_server();
    }

    async fn start_server(self) -> Result<(), Box<dyn std::error::Error>> {
        let runtime = tokio::runtime::Runtime::new().unwrap();

        runtime.block_on(async move {
            let addr = format!("0.0.0.0:{}", self.port).parse();
            let ccore = DriverManagerService::default();
            let ccore = DriverManagerServer::new(ccore);
            let mut server = Server::builder();
            println!("Running on port {}...", self.port);
            let router = server.add_service(ccore);
            router
                .serve_with_shutdown(addr.unwrap(), self.receiver.map(|_| ()))
                .await
        });
        Ok(())
    }

    pub fn stop_server(self) {
        println!("Stopping server...");
        self.sender.send(()).unwrap();
    }
}
