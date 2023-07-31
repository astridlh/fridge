use crate::utils::Date;

#[derive(Debug)]
pub struct FoodType {
    name: String,
    days_until_perished_open: i32,
}

impl FoodType {
    pub fn new(name: String, days_until_perished_open: i32) -> Self {
        Self {
            name,
            days_until_perished_open,
        }
    }

    // fn is_after_date(&self, date: Date) -> bool {
    //     true // TODO: implement
    // }
}

pub struct FoodItem {
    open: bool,
    epxiriy_date: Date,
}
