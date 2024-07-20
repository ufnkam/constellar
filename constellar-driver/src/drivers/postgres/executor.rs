use std::error::Error;
use std::ffi::{c_char, CString};
use std::ptr::null;
use libpq_sys::PGconn;
use crate::engine::DbResult;

pub trait Executor {
    unsafe fn execute<R: DbResult + Sized>(
        conn: *mut libpq_sys::PGconn,
        query: *const c_char,
    ) -> Result<R, Box<dyn std::error::Error>>;
}

pub struct PGPreparedStatementExecutor {}

pub struct PGStatementExecutor {}

impl Executor for PGPreparedStatementExecutor {
    unsafe fn execute<R: DbResult + Sized>(conn: *mut PGconn, query: *const c_char) -> Result<R, Box<dyn Error>> {
        let params: Vec<i32> = Vec::new();
        libpq_sys::PQprepare(
            conn,
            CString::new("stmt01")?.as_ptr(),
            query,
            0,
            null()
        );

        let res = libpq_sys::PQexecPrepared(
            conn,
            CString::new("stmt01")?.as_ptr(),
            0,
            null(),
            null(),
            null(),
            1
        );
        let pg_res = DbResult::new(Box::new(res));
        Ok(pg_res)
    }
}

impl Executor for PGStatementExecutor {
    unsafe fn execute<R: DbResult>(
        conn: *mut libpq_sys::PGconn,
        query: *const c_char,
    ) -> Result<R, Box<dyn Error>> {
        let params: Vec<i32> = Vec::new();
        let res = libpq_sys::PQexecParams(
            conn,
            query,
            0,
            null(),
            null(),
            null(),
            null(),
            1,
        );
        let pg_res = DbResult::new(Box::new(res));
        Ok(pg_res)
    }
}

fn _check_result_status(
    status: libpq_sys::ExecStatusType,
) -> Result<(), Box<dyn std::error::Error>> {
    match status {
        libpq_sys::ExecStatusType::PGRES_COMMAND_OK => Ok(()),
        libpq_sys::ExecStatusType::PGRES_TUPLES_OK => Ok(()),
        _ => panic!("Error executing query"),
    }
}
