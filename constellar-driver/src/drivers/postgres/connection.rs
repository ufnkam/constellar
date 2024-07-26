use crate::drivers::postgres::params::PgConnectionParams;
use crate::drivers::postgres::{statement_exec, PgBackend, PgResultWrapper, PreparedStatement};
use crate::engine::{
    Backend, Connection, ConnectionParams, ConnectionPool, ConnectionStatus, DbResult, ToSql,
};
use std::error::Error;
use std::ffi::{c_char, CStr, CString};
use std::string::String;

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
        let status = unsafe { libpq_sys::PQstatus(internal_conn) };

        match status {
            libpq_sys::ConnStatusType::CONNECTION_BAD => {
                let err_msg =
                    unsafe { CStr::from_ptr(libpq_sys::PQerrorMessage(internal_conn)).to_str()? };
                panic!("Connection failed: {}", err_msg);
            }
            _ => {}
        }
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
        params: &[&(dyn ToSql<PgBackend>)],
    ) -> Result<PgResultWrapper, Box<dyn std::error::Error>> {
        unsafe {
            if self.status != ConnectionStatus::Connected {
                panic!("Not connected");
            }

            let prepare = match self.connection_params.prepared_threshold {
                count if count > 0 => true,
                _ => false,
            };
            let c_query = CString::new(query).unwrap();

            if prepare {
                let mut prepared =
                    PreparedStatement::new(self.conn, c_query.as_ptr(), &mut self.prepared_cache);
                prepared.allocate(&params)?;
                let res = prepared.execute(&params)?;
                prepared.deallocate()?;
                Ok(res)
            } else {
                let res = statement_exec(self.conn, c_query.as_ptr())?;
                Ok(res)
            }
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
    use crate::engine::{Backend, Connection, ConnectionPool, DbResult, ToSql};

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

    async fn mock_table(
        conn: &mut PgConnection,
    ) -> Result<PgResultWrapper, Box<dyn std::error::Error>> {
        let mut query = "drop table if exists test_table cascade;";
        _ = conn.execute(query, &[]).await?;
        query = "create table test_table (id serial primary key, name text);";

        let res = conn.execute(query, &[]).await?;
        query = "insert into test_table (name) values ($1), ($2);";

        let int_param: i32 = 1;
        let res = conn.execute(query, &[&"xd".to_string(), &"Lol".to_string()]).await?;
        assert_eq!(res.affected, 2);
        Ok(res)
    }

    async fn clean_mock_table<B: Backend>(
        conn: &mut B::Connection,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let query = "drop table test_table;";
        conn.execute(query, &Vec::new()).await?;
        Ok(())
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
        let res: PgResultWrapper = conn.execute(query, &[]).await?;
        conn.close().await?;
        assert_eq!(res.row_count, 1);
        assert_eq!(res.column_count, 1);
        res.dispose()?;
        Ok(())
    }

    #[tokio::test]
    async fn test_results() -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = PgConnection::connect(mock_params()).await?;
        let res = mock_table(&mut conn).await?;
        println!(
            "{:?}, {:?}, {:?}",
            res.column_count, res.row_count, res.affected
        );
        res.dispose()?;
        clean_mock_table::<PgBackend>(&mut conn).await?;
        conn.close().await?;
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
