pub struct DataSource {
    host: &'static str,
    resource: &'static str,
}

impl DataSource {
    pub fn new(host: &'static str, resource: &'static str) -> Self {
        return DataSource { host, resource };
    }
}
