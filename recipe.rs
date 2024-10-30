use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub ingredients: Vec<String>,
    pub instuctions: Vec<String>,
    pub servings: u32,
}

impl Recipe {
    pub fn new(id: i32, name: String, ingredients: Vec<String>, instuctions: Vec<String>, servings: u32) -> Self {
        Recipe {
            id,
            name,
            ingredients,
            instuctions,
            servings,
        }
    }
}