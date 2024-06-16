use constellar_driver::client::cdriver::c_driver_client::CDriverClient;
use constellar_driver::client::cdriver::HealthCheckResponse;

pub async fn health() -> Result<(HealthCheckResponse), Box<dyn std::error::Error>>{
    let mut client = CDriverClient::connect("http://localhost:50051").await?;
    let res = constellar_driver::client::health_check(&mut client).await?;
    Ok(res)
}