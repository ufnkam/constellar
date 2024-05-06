use crate::sql::Column;

pub trait SqlDriver {
    fn get_table_schema<T: Column>(self, table: &'static str) -> Vec<T>;
}

pub trait Driver {
    fn get_name(self) -> String;
}
