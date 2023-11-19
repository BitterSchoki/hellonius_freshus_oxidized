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

impl Filters {
    pub fn apply(&self, ingredient: &Ingredient) -> bool {
        let mut ok = !self
            .food_groups
            .iter()
            .any(|fg| ingredient.food_groups.contains(fg));
        ok &= self
            .diet_goals
            .iter()
            .all(|dg| ingredient.diet_goals.contains(dg));
        ok &= self
            .special_diets
            .iter()
            .all(|sd| ingredient.special_diets.contains(sd));
        ok &= !self.avoid.iter().any(|&a| a == ingredient.id);
        ok
    }
}
