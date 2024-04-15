use constellar_core::server::server::run_server;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = run_server().await?;
    Ok(())
}
