use anyhow::anyhow;
use rocket::fairing::AdHoc;
use rocket::serde::Serialize;

mod crud;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Recipe {
    title: String,
    description: String,
    serves: i64,
    components: Vec<RecipeComponent>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct RecipeComponent {
    ingredient: Ingredient,
    amount: f64,
    unit: Unit,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Ingredient {
    title: String,
    description: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
enum Unit {
    Grams,
    Pieces,
    Milliliters, // and more ...
}

impl TryFrom<&str> for Unit {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "g" => Ok(Unit::Grams),
            "pcs" => Ok(Unit::Pieces),
            "ml" => Ok(Unit::Milliliters),
            unit => Err(anyhow!("Unknown unit {}", unit)),
        }
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Recipe routes", |rocket| async {
        rocket.mount("/recipes", routes![crud::read_recipe])
    })
}
