mod food;
mod storage;
mod utils;

use crate::food::FoodType;

fn main() {
    let egg = FoodType::new(String::from("egg"), i32::from(30));

    println!("{:?}", egg);
}
