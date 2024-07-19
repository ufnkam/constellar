use std::any::Any;
use std::error::Error;
use crate::engine::result::DbResult;

#[derive(Debug)]
pub struct PgResultWrapper {
    pub result: *mut libpq_sys::PGresult,
    pub row_count: usize,
    pub column_count: usize,
}

impl DbResult for PgResultWrapper {

    fn new(result: Box<dyn Any>) -> Self {
        let result = *result.downcast::<*mut libpq_sys::PGresult>().unwrap();
        let row_count = unsafe { count_rows(&result) };
        let column_count = unsafe { count_fields(&result) };
        return Self {
            result,
            row_count,
            column_count,
        };
    }
    fn get_row_count(&self) -> usize {
        return self.row_count;
    }

    fn dispose(&mut self) -> Result<(), Box<dyn Error>> {
        unsafe {
            libpq_sys::PQclear(self.result)
        }
        Ok(())
    }
}

unsafe fn count_fields(res: &*mut libpq_sys::PGresult) -> usize {
    let fields = libpq_sys::PQnfields(*res as *const libpq_sys::PGresult) as usize;
    return fields;
}

unsafe fn count_rows(res: &*mut libpq_sys::PGresult) -> usize {
    let rows = libpq_sys::PQntuples(*res as *const libpq_sys::PGresult) as usize;
    return rows;
}
