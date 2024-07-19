mod health;

pub mod cdriver {
    tonic::include_proto!("cdriver");
}
pub use cdriver::c_driver_client::CDriverClient;
pub use cdriver::HealthCheckResponse;
pub use health::health_check;