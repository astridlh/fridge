use crate::food::FoodItem;

pub enum StorageCategory {
    Cupboard,
    Fridge,
    Freezer,
}

pub struct Storage {
    name: String,
    category: StorageCategory,
    contents: Vec<FoodItem>,
}

impl Storage {
    pub fn new(name: String, category: StorageCategory) -> Self {
        Self {
            name,
            category,
            contents: vec![],
        }
    }
}
