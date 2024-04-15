use std::{thread::sleep, time::Duration};

use constellar_driver_manager::driver_manager::manager::DriverManagerRuntime;

fn main() {
    let rt = DriverManagerRuntime::default();
    sleep(Duration::new(10, 0));
    rt.stop_server()
}
