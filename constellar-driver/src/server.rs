use crate::engine::{AccessToken, DriverNativeDataSource, Query};
use crate::engine::{ConnectionParams, ConnectionStash};
use crate::grpc::cdriver::c_driver_server::{CDriver, CDriverServer};
use crate::grpc::cdriver::{
    Command, ConnectionInfo, DataSourceAccessToken, DataSourceInfo, HealthCheckResponse,
};
use arrow_flight::flight_service_server::{FlightService, FlightServiceServer};
use arrow_flight::{
    Action, ActionType, Criteria, Empty, FlightData, FlightDescriptor, FlightInfo,
    HandshakeRequest, HandshakeResponse, PollInfo, PutResult, Result as FlightResult, SchemaResult,
    Ticket,
};
use futures::stream::BoxStream;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio_stream::StreamExt;
use tokio_stream::{wrappers::ReceiverStream, Stream};
use tonic::transport::Server;
use tonic::{Request, Response, Status, Streaming};

//#[derive(Debug)]
pub struct CDriverService {
    stash: Arc<Mutex<ConnectionStash>>,
}

impl Default for CDriverService {
    fn default() -> Self {
        let stash = ConnectionStash::new();
        let locked_stash = Arc::new(Mutex::new(stash));

        CDriverService {
            stash: locked_stash,
        }
    }
}

#[tonic::async_trait]
impl CDriver for CDriverService {
    type ListDataSourcesStream = Pin<Box<dyn Stream<Item = Result<DataSourceInfo, Status>> + Send>>;

    async fn health_check(
        &self,
        request: Request<()>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        let res = HealthCheckResponse { living: true };
        Ok(Response::new(res))
    }

    async fn list_data_sources(
        &self,
        request: Request<()>,
    ) -> std::result::Result<tonic::Response<Self::ListDataSourcesStream>, tonic::Status> {
        // grpcurl TestCommand
        // grpcurl -plaintext -proto src/grpc/driver.proto localhost:50051 cdriver.CDriver/ListDataSources
        let (tx, rx) = mpsc::channel(4);
        let mut stream_values = Vec::new();

        let unlocked_stash = self
            .stash
            .lock()
            .map_err(|_| Status::internal("cannot lock stash"))?;

        for source in &unlocked_stash.deref().stash {
            let serialized_source = DataSourceInfo {
                name: source.get_name(),
                host: source.host.to_string(),
                port: "5432".to_string(),
                user: "xd".to_string(),
            };
            stream_values.push(serialized_source)
        }

        let mut stream = Box::pin(tokio_stream::iter(stream_values));
        tokio::spawn(async move {
            while let Some(item) = stream.next().await {
                tx.send(Ok(item.clone())).await.unwrap()
            }
        });

        let out_stream = ReceiverStream::new(rx);
        let res = Response::new(Box::pin(out_stream) as Self::ListDataSourcesStream);
        Ok(res)
    }

    async fn create_data_source(
        &self,
        request: Request<ConnectionInfo>,
    ) -> Result<Response<DataSourceAccessToken>, Status> {
        // grpcurl TestCommand
        // grpcurl -plaintext -d {driver: PostgreSQL, host: localhost, port: 9999, user: postgres_user, password: postgres_password, database: postgres} -proto src/grpc/driver.proto localhost:9999 cdriver.CDriver/CreateDataSource
        let body = request.get_ref();
        let params = ConnectionParams::from(body);
        let (dnds_res, name) = self
            .stash
            .lock()
            .map_err(|_| Status::internal("cannot lock stash"))?
            .deref_mut()
            .create_data_source(params, None)
            .expect("cannot generate access token");
        let token = DataSourceAccessToken {
            token: dnds_res.into(),
            name,
        };
        Ok(Response::new(token))
    }

    async fn get_data_source(
        &self,
        request: Request<DataSourceAccessToken>,
    ) -> Result<Response<DataSourceInfo>, Status> {
        // grpcurl TestCommand
        // grpcurl -plaintext -d '{name: localhost@postgres, token: 5872104779978545362}' -proto src/grpc/driver.proto localhost:9999 cdriver.CDriver/GetDataSource
        let body = request.get_ref();
        let info: DataSourceInfo = self
            .stash
            .lock()
            .map_err(|_| Status::internal("cannot lock stash"))?
            .deref()
            .get_data_source(&body.name, &AccessToken::from(&body.token))?
            .into();

        Ok(Response::new(info))
    }

    async fn execute_command(&self, request: Request<Command>) -> Result<Response<()>, Status> {
        // grpcurl TestCommand
        // grpcurl -plaintext -d '{"name": "localhost@postgres", "token": 5872104779978545362, "query": "select 1 as one, 2 as two"}' -proto src/grpc/driver.proto localhost:9999 cdriver.CDriver/ExecuteCommand
        let body = request.get_ref();
        let cmd = body.clone();
        let query = Query::new(cmd.query, ());

        let _ = self
            .stash
            .lock()
            .map_err(|_| Status::internal("cannot lock stash"))?
            .deref_mut()
            .get_data_source(&body.name, &AccessToken::from(&body.token))?
            .execute_command(&query);

        Ok(Response::new(()))
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

pub struct DriverServer {}

impl DriverServer {
    pub fn new() -> Self {
        DriverServer {}
    }

    pub async fn run_server(&self, port: &str) -> Result<(), Box<dyn std::error::Error>> {
        let addr = format!("0.0.0.0:{}", port).parse()?;

        let driver_service = CDriverService::default();
        let driver_server = CDriverServer::new(driver_service);

        println!("Running on port {}...", port);
        Server::builder()
            .add_service(driver_server)
            .serve(addr)
            .await?;
        Ok(())
    }
}
