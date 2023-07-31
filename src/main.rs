mod storage;

#[derive(Debug)]
struct Date {
    day: i8,
    month: i8,
    year: i32, // TODO: deal with leap years/dates
}

impl Date {
    fn new() -> Self {
        Self {
            day: 0,
            month: 0,
            year: 0,
        }
    } // TODO: implement
}

#[derive(Debug)]
struct Food {
    name: String,
    open: bool,
    expriry_date_unopened: Date,
    expiriy_date_opened: Date,
}

impl Food {
    fn new(name: String, expriry_date_unopened: Date, expiriy_date_opened: Date) -> Self {
        Self {
            name,
            open: false,
            expriry_date_unopened,
            expiriy_date_opened,
        }
    }

    fn is_after_date(&self, date: Date) -> bool {
        true // TODO: implement
    }
}

fn main() {
    let egg = Food::new(String::from("egg"), Date::new(), Date::new());

    println!("{:?}", egg);
}

