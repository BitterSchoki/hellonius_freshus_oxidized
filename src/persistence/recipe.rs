use crate::model::{Ingredient, Recipe, RecipeComponent};

pub async fn get_recipe(
    db: &mut sqlx::SqliteConnection,
    id: i64,
) -> Result<Option<Recipe>, sqlx::Error> {
    sqlx::query!(
        "SELECT title, serves, descr
        FROM recipes
        WHERE id = ?",
        id
    )
    .fetch_optional(db)
    .await
    .map(|r| {
        r.map(|r| Recipe {
            title: r.title,
            description: r.descr,
            serves: r.serves,
            components: vec![],
        })
    })
}

pub async fn get_recipe_components(
    db: &mut sqlx::SqliteConnection,
    id: i64,
) -> Result<Vec<RecipeComponent>, sqlx::Error> {
    let results = sqlx::query!(
        "SELECT ri.amount, ri.unit, i.title, i.descr, i.id
        FROM recipes r
            JOIN recipe_ingredients ri ON r.id = ri.recipe_id
            JOIN ingredients i ON i.id = ri.ingredient_id
        WHERE r.id = ?",
        id
    )
    .fetch_all(db)
    .await?;
    Ok(results
        .into_iter()
        .map(|r| RecipeComponent {
            ingredient: Ingredient {
                id: r.id,
                title: r.title,
                description: r.descr,
                ..Default::default()
            },
            amount: r.amount,
            unit: r.unit[..].into(),
        })
        .collect())
}
