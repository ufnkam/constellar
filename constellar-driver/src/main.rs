use constellar_driver::server::DriverServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = DriverServer::new();
    server.run_server().await?;
    Ok(())
}
