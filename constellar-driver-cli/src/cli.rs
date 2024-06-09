use constellar_driver::client::ClientWrapper;

pub struct CDriverCli {
    client: ClientWrapper,
}

impl CDriverCli {
    pub async fn new() -> Result<CDriverCli, Box<dyn std::error::Error>> {
        let client = ClientWrapper::connect("http://[::1]:50051").await?;
        Ok(CDriverCli { client })
    }
}
