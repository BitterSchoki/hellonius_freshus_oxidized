use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Ingredient {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub food_groups: Vec<FoodGroup>,
    pub diet_goals: Vec<DietGoal>,
    pub special_diets: Vec<SpecialDiet>,
}

impl Default for Ingredient {
    fn default() -> Self {
        Self {
            id: 0,
            title: String::new(),
            description: String::new(),
            food_groups: vec![],
            diet_goals: vec![],
            special_diets: vec![],
        }
    }
}

// Note that the From<i64> trait implementations for the enums have to match the ids in the database.

#[derive(Eq, PartialEq, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub enum FoodGroup {
    Lactose,
    Gluten,
    Histamines,
    Seafood,
}

impl From<i64> for FoodGroup {
    fn from(value: i64) -> Self {
        match value {
            1 => FoodGroup::Lactose,
            2 => FoodGroup::Gluten,
            3 => FoodGroup::Histamines,
            4 => FoodGroup::Seafood,
            _ => panic!("Unknown food group: {}", value),
        }
    }
}

#[derive(Eq, PartialEq, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub enum DietGoal {
    LowCarb,
    HighCarb,
    HighFat,
    CleanEating,
}

impl From<i64> for DietGoal {
    fn from(value: i64) -> Self {
        match value {
            1 => DietGoal::LowCarb,
            2 => DietGoal::HighCarb,
            3 => DietGoal::HighFat,
            4 => DietGoal::CleanEating,
            _ => panic!("Unknown diet goal: {}", value),
        }
    }
}

#[derive(Eq, PartialEq, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub enum SpecialDiet {
    Vegan,
    Vegetarian,
    Pescetarian,
    Keto,
}

impl From<i64> for SpecialDiet {
    fn from(value: i64) -> Self {
        match value {
            1 => SpecialDiet::Vegan,
            2 => SpecialDiet::Vegetarian,
            3 => SpecialDiet::Pescetarian,
            4 => SpecialDiet::Keto,
            _ => panic!("Unknown special diet: {}", value),
        }
    }
}
