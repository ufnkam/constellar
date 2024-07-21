use crate::drivers::postgres::executor::{Executor, PGStatementExecutor};
use crate::drivers::postgres::params::PgConnectionParams;
use crate::drivers::postgres::{PGPreparedStatementExecutor, PgBackend, PgResultWrapper};
use crate::engine::{Connection, ConnectionParams, ConnectionPool, ConnectionStatus, DbResult};
use std::error::Error;
use std::ffi::{c_char, CString};

#[derive(Clone)]
pub struct PgConnection {
    conn: *mut libpq_sys::PGconn,
    status: ConnectionStatus,
    connection_params: PgConnectionParams,
    prepared_cache: i32,
}

impl Connection<PgBackend> for PgConnection {
    async fn connect(params: PgConnectionParams) -> Result<Self, Box<dyn std::error::Error>> {
        let internal_conn = unsafe { libpq_sys::PQconnectdb(CString::new(params.uri())?.as_ptr()) };
        let conn = Self {
            conn: internal_conn,
            status: ConnectionStatus::Connected,
            connection_params: params,
            prepared_cache: 0,
        };
        Ok(conn)
    }

    async fn execute(
        &mut self,
        query: &str,
    ) -> Result<PgResultWrapper, Box<dyn std::error::Error>> {
        if self.status != ConnectionStatus::Connected {
            panic!("Not connected");
        }

        let prepare = match self.connection_params.prepared_threshold {
            count if count > 0 => true,
            _ => false,
        };
        let c_query = unsafe { CString::new(query).unwrap() };

        if prepare {
            let res = unsafe { PGPreparedStatementExecutor::execute(self.conn, c_query.as_ptr())? };
            Ok(res)
        } else {
            let res = unsafe { PGStatementExecutor::execute(self.conn, c_query.as_ptr())? };
            Ok(res)
        }
    }

    async fn close(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            libpq_sys::PQfinish(self.conn);
        }
        Ok(())
    }

    async fn cancel(&mut self) -> Result<(), Box<dyn Error>> {
        unsafe {
            let request = libpq_sys::PQgetCancel(self.conn);
            libpq_sys::PQcancel(request, CString::new("")?.as_ptr() as *mut c_char, 0);
            libpq_sys::PQfreeCancel(request);
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::drivers::postgres::connection::PgConnection;
    use crate::drivers::postgres::params::PgConnectionParams;
    use crate::drivers::postgres::result::PgResultWrapper;
    use crate::drivers::postgres::PgBackend;
    use crate::engine::{Connection, ConnectionPool, DbResult};

    fn mock_params() -> PgConnectionParams {
        return PgConnectionParams {
            username: &"postgres_user",
            password: &"postgres_password",
            dbname: &"postgres",
            host: &"localhost",
            port: &"9999",
            prepared_threshold: 5,
        };
    }

    #[tokio::test]
    async fn test_connection() -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = PgConnection::connect(mock_params()).await?;
        conn.close().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_execute() -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = PgConnection::connect(mock_params()).await?;
        let query = "select 1;";
        let params: Vec<i32> = Vec::new();
        let res: PgResultWrapper = conn.execute(query).await?;
        conn.close().await?;
        println!("RESULT={:?}", unsafe {
            libpq_sys::PQgetvalue(res.result, 0, 0)
        });
        res.dispose()?;
        Ok(())
    }

    #[tokio::test]
    async fn test_pool() -> Result<(), Box<dyn std::error::Error>> {
        let pool_size = 3;
        let mut pool: ConnectionPool<PgBackend> = ConnectionPool::new(pool_size, 5, mock_params());
        pool.open().await?;

        let mut conn_vec = Vec::new();
        for i in 0..pool_size {
            conn_vec.push(pool.get_conn());
        }

        assert_eq!(conn_vec.len(), pool_size as usize);
        assert_eq!(pool.connections.len(), pool_size as usize - conn_vec.len());
        pool.close().await?;
        Ok(())
    }

    #[test]
    fn test_session() {}
}
