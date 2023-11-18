use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Recipe {
    pub title: String,
    pub description: String,
    pub serves: i64,
    pub components: Vec<RecipeComponent>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RecipeComponent {
    pub ingredient: Ingredient,
    pub amount: f64,
    pub unit: Unit,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Ingredient {
    pub title: String,
    pub description: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub enum Unit {
    Grams,
    Pieces,
    Milliliters, // and more ...
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