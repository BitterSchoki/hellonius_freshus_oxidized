use rocket::serde::Serialize;

use super::ingredient::Ingredient;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Recipe {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub serves: i64,
    pub components: Vec<RecipeComponent>,
}

#[derive(Clone, Default, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RecipeComponent {
    pub ingredient: Ingredient,
    pub amount: f64,
    pub unit: Unit,
    pub was_replaced: bool,
}

#[derive(Clone, Copy, Serialize)]
#[serde(crate = "rocket::serde")]
pub enum Unit {
    Grams,
    Pieces,
    Milliliters, // and more ...
}

impl Default for Unit {
    fn default() -> Self {
        Unit::Grams
    }
}

impl From<&str> for Unit {
    fn from(value: &str) -> Self {
        match value {
            "g" => Unit::Grams,
            "pcs" => Unit::Pieces,
            "ml" => Unit::Milliliters,
            unit => panic!("Unknown unit: {}", unit),
        }
    }
}
