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

pub async fn get_full_recipe(
    db: &mut sqlx::SqliteConnection,
    id: i64,
) -> Result<Option<Recipe>, sqlx::Error> {
    if let Some(mut recipe) = get_recipe_with_components(db, id).await? {
        let mut components = Vec::with_capacity(recipe.components.len());
        for mut component in recipe.components.into_iter() {
            component.ingredient =
                crate::logic::ingredient::load_tags(db, component.ingredient).await?;
            components.push(component);
        }
        recipe.components = components;
        Ok(Some(recipe))
    } else {
        Ok(None)
    }
}
