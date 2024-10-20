use constellar_driver::cli::build_service_cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(build_service_cli().await?)
}
