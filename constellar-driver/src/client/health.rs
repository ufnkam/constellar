use crate::client::CDriverClient;
use crate::client::HealthCheckResponse;

pub async fn health_check(
    client: &mut CDriverClient<tonic::transport::Channel>,
) -> Result<HealthCheckResponse, Box<dyn std::error::Error>> {
    let request = tonic::Request::new(());
    let response = client.health_check(request).await?;
    println!("RESPONSE={:?}", response);
    Ok(response.into_inner())
}
