pub mod cli;
mod health;
use health::check_health;
mod client;

mod service;
pub use cli::{build_client_cli, build_service_cli};
use service::start_server;
