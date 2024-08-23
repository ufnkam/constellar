use crate::engine::ConnectionParams;
use std::hash::{DefaultHasher, Hash, Hasher};

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

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::engine::ConnectionParams;

    #[test]
    fn test_token() -> Result<(), Box<dyn std::error::Error>> {
        let params = ConnectionParams::new(
            "PostgreSQL",
            "localhost",
            &9999,
            "postgres_user",
            "postgres_password",
            "postgres",
            None,
        );

        let token = AccessToken::new(&params)?;
        assert_eq!(token.hash, 5872104779978545362);
        Ok(())
    }
}
