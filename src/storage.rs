use crate::Food;

pub enum StorageCategory {
    Cupboard,
    Fridge,
    Freezer
}

pub struct Storage {
    name: String,
    category: StorageCategory,
    contents: Vec<Food>
}

impl Storage {
    pub fn new(name: String, category: StorageCategory) -> Self {
        Self {
            name,
            category,
            contents: vec![]
        }
    }
}

