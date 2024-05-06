use crate::engine::connection::ConnectionParams;
use std::hash::{DefaultHasher, Hash, Hasher};

pub struct AccessToken {
    hash: u64,
}

impl AccessToken {
    pub fn new<P: ConnectionParams + Hash>(connection_params: &P) -> Self {
        let mut hasher = DefaultHasher::new();
        connection_params.hash(&mut hasher);
        AccessToken {
            hash: hasher.finish(),
        }
    }
}

impl PartialEq for AccessToken {
    fn eq(&self, other: &Self) -> bool {
        if !(self.hash == other.hash) {
            return false;
        }
        return true;
    }
    fn ne(&self, other: &Self) -> bool {
        if self.hash == other.hash {
            return true;
        }
        return false;
    }
}
