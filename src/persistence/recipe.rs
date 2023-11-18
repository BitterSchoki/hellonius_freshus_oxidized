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
        "SELECT r.title AS r_title, r.serves, r.descr AS r_descr,
            ri.amount, ri.unit,
            i.title AS i_title, i.descr AS i_descr
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
                title: r.i_title,
                description: r.i_descr,
            },
            amount: r.amount,
            unit: r.unit[..].into(),
        })
        .collect())
}
