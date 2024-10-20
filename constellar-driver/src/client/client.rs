use std::{str::FromStr, string::String};

use crate::grpc::cdriver::{
    c_driver_client::CDriverClient, c_driver_server::CDriver, ConnectionInfo,
};

pub struct Client {
    host: &'static str,
    port: &'static str,
    inner: CDriverClient<tonic::transport::Channel>,
}

impl Client {
    pub async fn new(host: &'static str, port: &'static str) -> Client {
        let inner = CDriverClient::connect(format!("http://{host}:{port}"))
            .await
            .expect("Cannot connect to server");
        Client { host, port, inner }
    }

    pub async fn check_health(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let request = tonic::Request::new(());
        let response = self.inner.health_check(request).await?;
        println!("RESPONSE={:?}", response);
        Ok(())
    }

    pub async fn create_data_source(
        &mut self,
        driver: String,
        host: String,
        port: u64,
        user: String,
        password: String,
        database: String,
        application_name: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conninfo = ConnectionInfo {
            driver,
            host,
            port,
            user,
            password,
            database,
            application_name,
        };
        let request = tonic::Request::new(conninfo);
        let _ = self.inner.create_data_source(request).await?;
        Ok(())
    }
}
