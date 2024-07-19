use std::error::Error;
use crate::engine::connection::{Connection, ConnectionParams, ConnectionStatus};
use crate::engine::result::{DbResult};
use crate::engine::typing::{FromSql, ToSql};
use std::ffi::{c_char, CString};
use libpq_sys::PGcancel;
use crate::drivers::postgres::params::PgConnectionParams;
use crate::drivers::postgres::executor::{ PGStatementExecutor, Executor };
use crate::drivers::postgres::PGPreparedStatementExecutor;


struct PgConnection {
    conn: *mut libpq_sys::PGconn,
    status: ConnectionStatus,
    connection_params: PgConnectionParams,
}

impl Connection<PgConnectionParams> for PgConnection {
    async fn connect(params: PgConnectionParams) -> Result<Self, Box<dyn std::error::Error>> {
        let internal_conn = unsafe { libpq_sys::PQconnectdb(CString::new(params.uri())?.as_ptr()) };
        let conn = Self {
            conn: internal_conn,
            status: ConnectionStatus::Connected,
            connection_params: params,
        };
        Ok(conn)
    }

    async fn execute<R: DbResult + Sized>(
        &mut self,
        query: &str,
    ) -> Result<R, Box<dyn std::error::Error>> {
        if self.status != ConnectionStatus::Connected {
            panic!("Not connected");
        }

        let prepare = self
            .connection_params
            .allow_prepared_statements
            .unwrap_or(true);
        let c_query = unsafe { CString::new(query).unwrap() };

        if prepare {
            let res:R = unsafe { PGPreparedStatementExecutor::execute(self.conn, c_query.as_ptr())? };
            Ok(res)
        } else {
            let res:R = unsafe { PGStatementExecutor::execute(self.conn, c_query.as_ptr())? };
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
    use crate::engine::connection::Connection;

    fn mock_params() -> PgConnectionParams {
        return PgConnectionParams {
            username: &"postgres_user",
            password: &"postgres_password",
            dbname: &"postgres",
            host: &"localhost",
            port: &"9999",
            allow_prepared_statements: Some(true),
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
        let xd: PgResultWrapper = conn.execute(query).await?;
        conn.close();
        println!("RESULT={:?}", unsafe{ libpq_sys::PQgetvalue(xd.result, 0,0 )});
        Ok(())
    }

    // #[test]
    // fn test_pool() -> Result<(), Box<dyn std::error::Error>> {
    //     let mut pool: ConnectionPool<PgConnection, PgConnectionParams> = ConnectionPool::new(3, 30, mock_params());
    //     pool.open()?;
    //     let mut sample_conn = pool.get_conn();
    //     sleep(Duration::new(10, 0));
    //     sample_conn.close()?;
    //     Ok(())
    // }

    #[test]
    fn test_session() {}
}
