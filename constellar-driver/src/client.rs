pub mod cdriver {
    tonic::include_proto!("cdriver");
}
use cdriver::c_driver_client::CDriverClient;
use cdriver::HealthCheckResponse;


struct Client {}

impl Client {}


pub async fn health_check(
    client: &mut CDriverClient<tonic::transport::Channel>,
) -> Result<HealthCheckResponse, Box<dyn std::error::Error>> {
    let request = tonic::Request::new(());
    let response = client.health_check(request).await?;
    println!("RESPONSE={:?}", response);
    Ok(response.into_inner())
}
