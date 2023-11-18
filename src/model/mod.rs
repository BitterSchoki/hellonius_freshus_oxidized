mod ingredient;
mod recipe;

use rocket::serde::Deserialize;

pub use ingredient::*;
pub use recipe::*;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Filters {
    pub keyword: String,

    pub food_groups: Vec<FoodGroup>,
    pub diet_goals: Vec<DietGoal>,
    pub special_diets: Vec<SpecialDiet>,

    pub deadly: Vec<i64>,
    pub avoid: Vec<i64>,
}
