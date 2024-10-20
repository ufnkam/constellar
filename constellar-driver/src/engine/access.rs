use crate::engine::ConnectionParams;
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Clone, Copy)]
pub struct AccessToken {
    hash: u64,
}

impl AccessToken {
    pub fn new(connection_params: &ConnectionParams) -> Result<Self, Box<dyn std::error::Error>> {
        let mut hasher = DefaultHasher::new();
        connection_params.hash(&mut hasher);
        Ok(AccessToken {
            hash: hasher.finish(),
        })
    }
}

impl PartialEq for AccessToken {
    fn eq(&self, other: &Self) -> bool {
        if !(self.hash == other.hash) {
            return false;
        }
        true
    }
    fn ne(&self, other: &Self) -> bool {
        if self.hash == other.hash {
            return true;
        }
        false
    }
}

impl Into<u64> for AccessToken {
    fn into(self) -> u64 {
        self.hash
    }
}
impl Into<u64> for &AccessToken {
    fn into(self) -> u64 {
        self.hash
    }
}

impl From<&u64> for AccessToken {
    fn from(value: &u64) -> Self {
        Self {
            hash: value.clone(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::engine::ConnectionParams;

    #[test]
    fn test_token() -> Result<(), Box<dyn std::error::Error>> {
        let params = ConnectionParams::new(
            "PostgreSQL".to_string(),
            "localhost".to_string(),
            "9999".parse().expect("not a number"),
            "postgres_user".to_string(),
            "postgres_password".to_string(),
            "postgres".to_string(),
            None,
        );

        let token = AccessToken::new(&params)?;
        assert_eq!(token.hash, 5872104779978545362);
        Ok(())
    }
}
