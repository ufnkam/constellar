use std::hash::{Hash, Hasher};
use crate::engine::ConnectionParams;

#[derive(Clone)]
pub struct PgConnectionParams {
    pub username: &'static str,
    pub password: &'static str,
    pub dbname: &'static str,
    pub host: &'static str,
    pub port: &'static str,
    pub prepared_threshold: i32
}

impl ConnectionParams for PgConnectionParams {
    fn uri(&self) -> String {
        return format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.dbname
        );
    }

    fn from_uri(uri: &str) -> Self {
        return Self {
            username: "postgres_user",
            password: "postgres_password",
            dbname: "postgres",
            host: "localhost",
            port: "9999",
            prepared_threshold: 5
        };
    }

    fn get_backend(&self) -> &'static str {
        return "postgres";
    }

    fn get_resource(&self) -> &'static str {
        return self.dbname;
    }

    fn get_host(&self) -> &'static str {
        return self.host;
    }
}

impl Hash for PgConnectionParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uri().hash(state);
    }
}
