use std::hash::Hash;
use constellar_driver::driver::Driver;
use constellar_driver::engine::connection::{Connection, ConnectionParams};
use constellar_driver::server::DriverServer;

pub async fn start_server<D, C, P>(server: DriverServer<D, C, P>) -> Result<(), Box<dyn std::error::Error>>
where D: Driver<C, P>,
        P: ConnectionParams + Hash + Clone,
        C: Connection<P> + Copy
{
    server.run_server().await?;
    Ok(())
}