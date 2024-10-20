use crate::server::DriverServer;
use crate::{cli::start_server, Client};
use clap::{Arg, ArgAction, Command};

static VERSION: &str = "0.1.0";

pub async fn build_client_cli() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Command::new("constellar-driver-client")
        .version(VERSION)
        .arg(
            Arg::new("host")
                .long("host")
                .action(ArgAction::Set)
                .required(true),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .action(ArgAction::Set)
                .required(true),
        )
        .subcommand(Command::new("health").about("Checks health of given server"))
        .subcommand(
            Command::new("data-source").subcommand(
                Command::new("create")
                    .arg(
                        Arg::new("host")
                            .long("host")
                            .action(ArgAction::Set)
                            .required(true),
                    )
                    .arg(
                        Arg::new("port")
                            .long("port")
                            .action(ArgAction::Set)
                            .required(true),
                    )
                    .arg(
                        Arg::new("driver")
                            .long("driver")
                            .action(ArgAction::Set)
                            .required(true),
                    )
                    .arg(
                        Arg::new("username")
                            .long("username")
                            .action(ArgAction::Set)
                            .required(true),
                    )
                    .arg(
                        Arg::new("password")
                            .long("password")
                            .action(ArgAction::Set)
                            .required(true),
                    )
                    .arg(
                        Arg::new("database")
                            .long("database")
                            .action(ArgAction::Set)
                            .required(true),
                    ),
            ),
        )
        .get_matches();
    // let host = cli.get_one::<String>("host").expect("Host not provided");
    // let port = cli.get_one::<String>("port").expect("Port not provided");
    let mut client = Client::new("localhost", "8090").await;

    match cli.subcommand() {
        Some(("health", sub_matches)) => {
            match client.check_health().await {
                Ok(_) => {
                    println!("I'M ALIVE")
                }
                Err(e) => println!("Error: {}", e),
            };

            Ok(())
        }
        Some(("data-source", sub_matches)) => {
            println!("entering");
            match sub_matches.subcommand() {
                Some(("create", sub_matches)) => {
                    let host = sub_matches
                        .get_one::<String>("host")
                        .expect("host is required")
                        .clone();
                    let port = sub_matches
                        .get_one::<String>("port")
                        .expect("port is required");
                    let driver = sub_matches
                        .get_one::<String>("driver")
                        .unwrap_or(&"PostgreSQL".to_string())
                        .clone();
                    let username = sub_matches
                        .get_one::<String>("username")
                        .expect("username is required")
                        .clone();
                    let password = sub_matches
                        .get_one::<String>("password")
                        .expect("username is required")
                        .clone();
                    let database = sub_matches
                        .get_one::<String>("password")
                        .unwrap_or(&"postgres".to_string())
                        .clone();
                    let application_name = "TwojaStara".to_string();

                    println!("creating data source for {:?}:{:?}", host, port);

                    client
                        .create_data_source(
                            driver,
                            host,
                            port.clone().parse()?,
                            username,
                            password,
                            database.clone(),
                            application_name.clone(),
                        )
                        .await?
                }
                _ => unimplemented!(),
            }
            Ok(())
        }
        _ => unimplemented!(),
    }
}

pub async fn build_service_cli() -> Result<(), Box<dyn std::error::Error>> {
    let server = DriverServer::new();
    let cli = Command::new("constellar-driver-service")
        .version(VERSION)
        .subcommand(
            Command::new("start").arg(
                Arg::new("port")
                    .short('p')
                    .long("port")
                    .default_value("50051".to_string())
                    .action(ArgAction::Set),
            ),
        )
        .get_matches();

    match cli.subcommand() {
        Some(("start", sub_matches)) => {
            let port = sub_matches
                .get_one::<String>("port")
                .expect("Port is expected");
            start_server(server, &port).await?;
            Ok(())
        }
        _ => unimplemented!(),
    }
}
