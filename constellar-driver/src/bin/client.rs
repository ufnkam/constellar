use constellar_driver::cli::build_client_cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(build_client_cli().await?)
}
