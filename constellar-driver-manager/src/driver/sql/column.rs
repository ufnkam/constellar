pub trait Column {
    fn new() -> Self;
    fn get_name(self) -> &'static str;
    fn get_data_type(self) -> &'static str;
}

pub trait ToArrow {}
