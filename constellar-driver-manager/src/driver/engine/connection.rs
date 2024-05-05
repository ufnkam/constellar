pub trait ConnectionParams {
    fn uri(&self) -> String;
    fn from_uri(self, uri: &str) -> Self;
    fn get_backend(&self) -> &str;
}

pub trait Connection<C: ConnectionParams> {
    fn connect(params: &C) -> Self;
}
pub trait ConnectionAdapter {}
