pub trait ConnectionParams {
    fn uri(&self) -> String;
    fn from_uri(self, uri: &str) -> Self;
    fn get_backend(&self) -> &str;
    fn get_resource(&self) -> &str;
    fn get_host(&self) -> &str;
}

pub trait Connection<P: ConnectionParams> {
    fn connect(params: &P) -> Self;
}
