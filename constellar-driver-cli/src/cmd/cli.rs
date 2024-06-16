use std::hash::Hash;

use clap::{Arg, Command};
use constellar_driver::driver::Driver;
use constellar_driver::engine::connection::{Connection, ConnectionParams};
use constellar_driver::server::DriverServer;
use crate::cmd::health::health;

use crate::cmd::server::{start_server};

fn info(msg_type: &str) {
    if msg_type.ne("basic") {
        println!("other", );
        return;
    }
    println!("This is the driver cmd");
}

pub async fn build_cli<D, C, P>(server: DriverServer<D, C, P>) -> Result<(), Box<dyn std::error::Error>>
where D: Driver<C, P>,
        P: ConnectionParams + Hash,
        C: Connection<P> + Copy
{
    let cli = Command::new("driver")
        .version("0.1.0")
        .subcommand(
            Command::new("info")
                .about("Prints information")
                .arg(Arg::new("msg-type").long("msg-type").short('t')),
        )
        .subcommand(
            Command::new("start")
                .about("Starts driver server")
        )
        .subcommand(
            Command::new("health")
                .about("checks health")
        )
        .get_matches();

    match cli.subcommand() {
        Some(("info", sub_matches)) => {
            let msg_type = sub_matches
                .get_one::<String>("msg-type")
                .map(|s| s.as_str())
                .unwrap();
            info(msg_type);
            Ok(())
        },
        Some(("start", sub_matches )) => {
            start_server(server).await?;
            Ok(())
        },
        Some(("health", sub_matches)) => {
            println!("{:?}", health().await?);
            Ok(())
        }
        _ => unimplemented!(),
    }
}
