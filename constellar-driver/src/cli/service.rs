use crate::server::DriverServer;

pub async fn start_server(
    server: DriverServer,
    port: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    server.run_server(port).await?;
    Ok(())
}
