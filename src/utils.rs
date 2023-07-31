#[derive(Debug)]
pub struct Date {
    day: i8,
    month: i8,
    year: i32, // TODO: deal with leap years/dates
}

impl Date {
    pub fn new() -> Self {
        Self {
            day: 0,
            month: 0,
            year: 0,
        }
    } // TODO: implement
}
