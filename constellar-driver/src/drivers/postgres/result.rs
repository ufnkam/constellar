use crate::engine::DbResult;
use std::any::Any;
use std::error::Error;
use std::ffi::CStr;

#[derive(Debug)]
pub struct PgResultWrapper {
    pub result: *mut libpq_sys::PGresult,
    pub row_count: usize,
    pub column_count: usize,
    pub affected: usize,
}

impl DbResult for PgResultWrapper {
    fn new(result: Box<dyn Any>) -> Self {
        let result = *result.downcast::<*mut libpq_sys::PGresult>().unwrap();
        let row_count = unsafe { count_rows(&result) };
        let column_count = unsafe { count_fields(&result) };
        let affected = unsafe { count_affected(&result) };
        return Self {
            result,
            row_count,
            column_count,
            affected,
        };
    }
    fn get_row_count(&self) -> usize {
        return self.row_count;
    }

    fn dispose(&self) -> Result<(), Box<dyn Error>> {
        unsafe {
            libpq_sys::PQclear(self.result);
        }
        Ok(())
    }
}

unsafe fn count_fields(res: &*mut libpq_sys::PGresult) -> usize {
    let fields = libpq_sys::PQnfields(*res) as usize;
    return fields;
}

unsafe fn count_rows(res: &*mut libpq_sys::PGresult) -> usize {
    let rows = libpq_sys::PQntuples(*res) as usize;
    return rows;
}

unsafe fn count_affected(res: &*mut libpq_sys::PGresult) -> usize {
    let affected = CStr::from_ptr(libpq_sys::PQcmdTuples(*res))
        .to_str()
        .unwrap();

    let empty = affected.is_empty();

    if empty {
        return 0;
    }
    return affected
        .parse::<u64>()
        .expect("Could not parse affected rows") as usize;
}
