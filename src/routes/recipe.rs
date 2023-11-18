use rocket::serde;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket_db_pools::Connection;

use crate::db;

use crate::logic;
use crate::model::RecipeComponent;
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

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Replacement {
    component: RecipeComponent,
    filters: Filters,
}

#[post("/replace", data = "<replacement>")]
pub async fn replace_ingredient(
    mut db: Connection<db::Recipes>,
    replacement: Json<Replacement>,
) -> Option<Json<Vec<RecipeComponent>>> {
    let replacements =
        logic::recipe::find_replacements(&mut db, &replacement.component, &replacement.filters)
            .await
            .ok()?;
    Some(replacements.into())
}
