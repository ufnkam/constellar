use std::any::Any;

pub trait DbResult {
    fn new(result: Box<dyn Any>) -> Self;
    fn get_row_count(&self) -> usize;

    fn dispose(&self) -> Result<(), Box<dyn std::error::Error>>;
}

