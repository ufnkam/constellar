pub mod cdriver {
    tonic::include_proto!("cdriver");
}

use crate::server::cdriver::HealthCheckResponse;
use arrow_flight::flight_service_server::{FlightService, FlightServiceServer};
use arrow_flight::{
    Action, ActionType, Criteria, Empty, FlightData, FlightDescriptor, FlightInfo,
    HandshakeRequest, HandshakeResponse, PollInfo, PutResult, SchemaResult, Ticket, Result as FlightResult
};
use cdriver::c_driver_server::{CDriver, CDriverServer};
use std::env;
use std::hash::Hash;
use tonic::transport::Server;
use tonic::{Request, Response, Status, Streaming};
use futures::stream::BoxStream;
use crate::engine::ConnectionStash;

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

#[tonic::async_trait]
impl FlightService for CDriverService {
    type HandshakeStream = BoxStream<'static, Result<HandshakeResponse, Status>>;
    async fn handshake(
        &self,
        request: Request<Streaming<HandshakeRequest>>,
    ) -> Result<Response<Self::HandshakeStream>, Status> {
        todo!()
    }
    type ListFlightsStream = BoxStream<'static, Result<FlightInfo, Status>>;
    async fn list_flights(
        &self,
        request: Request<Criteria>,
    ) -> Result<Response<Self::ListFlightsStream>, Status> {
        todo!()
    }
    async fn get_flight_info(
        &self,
        request: Request<FlightDescriptor>,
    ) -> Result<Response<FlightInfo>, Status> {
        todo!()
    }
    async fn poll_flight_info(
        &self,
        request: Request<FlightDescriptor>,
    ) -> Result<Response<PollInfo>, Status> {
        todo!()
    }
    async fn get_schema(
        &self,
        request: Request<FlightDescriptor>,
    ) -> Result<Response<SchemaResult>, Status> {
        todo!()
    }
    type DoGetStream = BoxStream<'static, Result<FlightData, Status>>;

    async fn do_get(
        &self,
        request: Request<Ticket>,
    ) -> Result<Response<Self::DoGetStream>, Status> {
        todo!()
    }

    type DoPutStream = BoxStream<'static, Result<PutResult, Status>>;

    async fn do_put(
        &self,
        request: Request<Streaming<FlightData>>,
    ) -> Result<Response<Self::DoPutStream>, Status> {
        todo!()
    }

    type DoExchangeStream = BoxStream<'static, Result<FlightData, Status>>;

    async fn do_exchange(
        &self,
        request: Request<Streaming<FlightData>>,
    ) -> Result<Response<Self::DoExchangeStream>, Status> {
        todo!()
    }

    type DoActionStream = BoxStream<'static, Result<FlightResult, Status>>;

    async fn do_action(
        &self,
        request: Request<Action>,
    ) -> Result<Response<Self::DoActionStream>, Status> {
        todo!()
    }

    type ListActionsStream = BoxStream<'static, Result<ActionType, Status>>;

    async fn list_actions(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<Self::ListActionsStream>, Status> {
        todo!()
    }
}


pub struct DriverServer {
    stash: ConnectionStash
}

impl DriverServer {
    pub fn new() -> Self {
        DriverServer { stash: ConnectionStash::new() }
    }

    pub async fn run_server(&self) -> Result<(), Box<dyn std::error::Error>> {
        let port = env::var("PORT").unwrap_or("50051".to_string());
        let addr = format!("0.0.0.0:{}", port).parse()?;

        let driver_service = CDriverService::default();
        let driver_server = CDriverServer::new(driver_service);

        println!("Running on port {}...", port);
        Server::builder().add_service(driver_server).serve(addr).await?;
        Ok(())
    }
}
