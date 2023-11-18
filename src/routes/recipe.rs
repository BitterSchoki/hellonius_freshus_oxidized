use rocket::serde::{json::Json, Deserialize};
use rocket_db_pools::Connection;

use crate::db;

use crate::logic;
use crate::model::{DietGoal, FoodGroup, Recipe, SpecialDiet};

#[get("/<id>")]
pub async fn get_recipe(mut db: Connection<db::Recipes>, id: i64) -> Option<Json<Recipe>> {
    let recipe = logic::recipe::get_full_recipe(&mut *db, id).await.ok()??;
    Some(recipe.into())
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Filters {
    food_groups: Vec<FoodGroup>,
    diet_goals: Vec<DietGoal>,
    special_diets: Vec<SpecialDiet>,
}

#[post("/filter", data = "<filters>")]
pub async fn filtered_recipes(
    mut db: Connection<db::Recipes>,
    filters: Json<Filters>,
) -> Option<Json<Vec<Recipe>>> {
    let recipes = logic::recipe::filter_recipes(
        &mut *db,
        &filters.food_groups,
        &filters.diet_goals,
        &filters.special_diets,
    )
    .await
    .ok()?;
    Some(recipes.into())
}
