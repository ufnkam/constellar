use clap::Command;

use crate::client::Client;

pub async fn check_health(client: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
    let _ = client.check_health().await?;
    Ok(())
}

fn health_cmd() -> clap::Command {
    let cmd = Command::new("health").about("Checks if the target server is alive");

    cmd
}
