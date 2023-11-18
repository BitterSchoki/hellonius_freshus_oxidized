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
    amount: Amount,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Ingredient {
    title: String,
    description: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
enum Amount {
    Grams(f64),
    Pieces(f64),
    Milliliters(f64), // and more ...
}

impl TryFrom<(f64, &str)> for Amount {
    type Error = anyhow::Error;

    fn try_from(value: (f64, &str)) -> Result<Self, Self::Error> {
        match value {
            (v, "g") => Ok(Amount::Grams(v)),
            (v, "pcs") => Ok(Amount::Pieces(v)),
            (v, "ml") => Ok(Amount::Milliliters(v)),
            (_, unit) => Err(anyhow!("Unknown unit {}", unit)),
        }
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Recipe routes", |rocket| async {
        rocket.mount("/recipes", routes![crud::read_recipe])
    })
}
