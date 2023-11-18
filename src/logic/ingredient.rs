use crate::model::Ingredient;
use crate::persistence::ingredient as ingredient_db;

pub async fn get_ingredient_with_tags(
    db: &mut sqlx::SqliteConnection,
    id: i64,
) -> Result<Option<Ingredient>, sqlx::Error> {
    let ingredient = ingredient_db::get_ingredient(db, id).await?;
    let (groups, goals, diets) = ingredient_db::get_ingredient_tags(db, id).await?;
    Ok(ingredient.map(|i| Ingredient {
        food_groups: groups,
        diet_goals: goals,
        special_diets: diets,
        ..i
    }))
}
