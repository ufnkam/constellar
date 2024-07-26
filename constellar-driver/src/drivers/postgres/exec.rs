use crate::drivers::postgres::{PgBackend, PgResultWrapper};
use crate::engine::{DbResult, ToSql};
use clap::builder::Str;
use libpq_sys::{Oid, PGconn, PQresultErrorMessage};
use std::error::Error;
use std::ffi::{c_char, c_int, CStr, CString};
use std::fmt::format;
use std::ops::{Add, Sub};
use std::ptr::null;
use std::slice::{from_raw_parts, from_raw_parts_mut};
use std::str::from_utf8;
use arrow::datatypes::ToByteSlice;
use bytes::BytesMut;

// pub trait Executor {
//     unsafe fn execute<R: DbResult + Sized>(
//         conn: *mut libpq_sys::PGconn,
//         query: *const c_char,
//     ) -> Result<R, Box<dyn std::error::Error>>;
// }

// impl Executor for PGPreparedStatementExecutor {
//     unsafe fn execute<R: DbResult + Sized>(
//         conn: *mut PGconn,
//         query: *const c_char,
//     ) -> Result<R, Box<dyn Error>> {
//         let params: Vec<i32> = Vec::new();
//         libpq_sys::PQprepare(conn, CString::new("stmt01")?.as_ptr(), query, 0, null());
//
//         let res = libpq_sys::PQexecPrepared(
//             conn,
//             CString::new("stmt01")?.as_ptr(),
//             0,
//             null(),
//             null(),
//             null(),
//             1,
//         );
//         let pg_res = DbResult::new(Box::new(res));
//         let status = libpq_sys::PQresultStatus(res);
//         match status {
//             libpq_sys::ExecStatusType::PGRES_COMMAND_OK => Ok(pg_res),
//             libpq_sys::ExecStatusType::PGRES_TUPLES_OK => Ok(pg_res),
//             _ => {
//                 let msg = CStr::from_ptr(PQresultErrorMessage(res)).to_str();
//                 panic!("{:?}", msg)
//             }
//         }
//     }
// }

pub struct PreparedStatement<'a> {
    conn: *mut libpq_sys::PGconn,
    name: String,
    query: *const c_char,
    prepared_cache: &'a mut i32,
}

impl<'a> PreparedStatement<'a> {
    pub fn new(
        conn: *mut libpq_sys::PGconn,
        query: *const c_char,
        prepared_cache: &'a mut i32,
    ) -> Self {
        Self {
            conn,
            name: String::new(),
            query,
            prepared_cache,
        }
    }
    pub unsafe fn allocate(&mut self, params: &[&(dyn ToSql<PgBackend>)] ) -> Result<(), Box<dyn std::error::Error>> {
        let name = format!("stmt_{}", self.prepared_cache.add(1));
        let param_types = params
            .iter()
            .map(|p| p.to_sql().1 as Oid)
            .collect::<Vec<Oid>>();

        libpq_sys::PQprepare(
            self.conn,
            CString::new(&*name)?.as_ptr(),
            self.query,
            params.len() as c_int,
            param_types.into_boxed_slice().as_ptr(),
        );
        self.name = name;
        self.prepared_cache.add(1);
        validate_prepared_statement(self.conn, CString::new(&*self.name)?.as_ptr())?;
        Ok(())
    }

    pub unsafe fn execute(
        &self,
        params: &[&(dyn ToSql<PgBackend>)],
    ) -> Result<PgResultWrapper, Box<dyn std::error::Error>> {
        let param_count: i32 = params.len() as i32;

        let mut param_formats = Vec::new();
        for i in 0..param_count {
            param_formats.push(1 as c_int)
        };

        let param_sizes = params
            .iter()
            .map(|p| p.to_sql().0.len() as c_int)
            .collect::<Vec<c_int>>();
        let buffered_params  = params
            .iter()
            .map(|p| p.to_sql().0.as_ptr())
            .collect::<Vec<*const u8>>();

        if buffered_params.len() > 0 {
            // let some_param = params[0].to_sql().0.to_byte_slice();
            let xd = from_utf8(from_raw_parts(params[0].to_sql().0.as_ptr(), 2));
            println!();
        }

        let res = libpq_sys::PQexecPrepared(
            self.conn,
            CString::new(&*self.name)?.as_ptr(),
            param_count as c_int,
            buffered_params.as_ptr() as *const *const c_char,
            param_sizes.as_ptr() as *const c_int,
            param_formats.as_ptr(),
            1,
        );

        check_result_status(res)?;
        let pg_res = PgResultWrapper::new(Box::new(res));
        Ok(pg_res)
    }
    pub unsafe fn deallocate(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let query = format!("deallocate {};", self.name);
        statement_exec(self.conn, CString::new(query)?.as_ptr())?;
        self.prepared_cache.sub(1);
        Ok(())
    }
}

pub unsafe fn statement_exec(
    conn: *mut libpq_sys::PGconn,
    query: *const c_char,
) -> Result<PgResultWrapper, Box<dyn std::error::Error>> {
    let params: Vec<i32> = Vec::new();
    let res = libpq_sys::PQexecParams(conn, query, 0, null(), null(), null(), null(), 1);
    check_result_status(res)?;
    let pg_res = DbResult::new(Box::new(res));
    Ok(pg_res)
}

unsafe fn check_result_status(
    res: *mut libpq_sys::PGresult,
) -> Result<(), Box<dyn std::error::Error>> {
    let status = libpq_sys::PQresultStatus(res);
    match status {
        libpq_sys::ExecStatusType::PGRES_COMMAND_OK => Ok(()),
        libpq_sys::ExecStatusType::PGRES_TUPLES_OK => Ok(()),
        _ => {
            let msg = CStr::from_ptr(PQresultErrorMessage(res)).to_str();
            panic!("{:?}", msg)
        }
    }
}

unsafe fn validate_prepared_statement(
    conn: *mut PGconn,
    name: *const c_char,
) -> Result<(), Box<dyn Error>> {
    let status = libpq_sys::PQdescribePrepared(conn, name);
    check_result_status(status)?;
    Ok(())
}
