use crate::model::Recipe;
use crate::persistence::recipe as recipe_db;

pub async fn get_recipe_with_components(
    db: &mut sqlx::SqliteConnection,
    id: i64,
) -> Result<Option<Recipe>, sqlx::Error> {
    let recipe = recipe_db::get_recipe(db, id).await?;
    let components = recipe_db::get_recipe_components(db, id).await?;
    Ok(recipe.map(|r| Recipe { components, ..r }))
}
