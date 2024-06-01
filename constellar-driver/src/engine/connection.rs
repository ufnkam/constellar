pub trait ConnectionParams {
    fn uri(&self) -> String;
    fn from_uri(self, uri: &str) -> Self;
    fn get_backend(&self) -> &'static str;
    fn get_resource(&self) -> &'static str;
    fn get_host(&self) -> &'static str;
}

pub trait Connection<P: ConnectionParams> {
    fn connect(params: &P) -> Self;
    fn close();
}
