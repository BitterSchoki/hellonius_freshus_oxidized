use rocket::serde::json::Json;
use rocket_db_pools::Connection;

use crate::db;

use crate::logic;
use crate::model::{Filters, Recipe};

#[get("/<id>")]
pub async fn get_recipe(mut db: Connection<db::Recipes>, id: i64) -> Option<Json<Recipe>> {
    let recipe = logic::recipe::get_full_recipe(&mut db, id).await.ok()??;
    Some(recipe.into())
}

#[post("/filter", data = "<filters>")]
pub async fn filtered_recipes(
    mut db: Connection<db::Recipes>,
    filters: Json<Filters>,
) -> Option<Json<Vec<Recipe>>> {
    let recipes = logic::recipe::filter_recipes(&mut db, &filters.into_inner())
        .await
        .ok()?;
    Some(recipes.into())
}
