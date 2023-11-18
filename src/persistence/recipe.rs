use crate::model::{Ingredient, Recipe, RecipeComponent};

pub async fn get_recipe(
    db: &mut sqlx::SqliteConnection,
    id: i64,
) -> Result<Option<Recipe>, sqlx::Error> {
    sqlx::query!(
        "SELECT id, title, serves, descr, image_url
        FROM recipes
        WHERE id = ?",
        id
    )
    .fetch_optional(db)
    .await
    .map(|r| {
        r.map(|r| Recipe {
            id: r.id,
            title: r.title,
            description: r.descr,
            serves: r.serves,
            components: vec![],
            image_url: r.image_url,
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
            ..Default::default()
        })
        .collect())
}

pub async fn all_recipes(db: &mut sqlx::SqliteConnection) -> Result<Vec<Recipe>, sqlx::Error> {
    let recipes = sqlx::query!(
        "SELECT id, title, serves, descr, image_url
        FROM recipes"
    )
    .fetch_all(db)
    .await?
    .into_iter()
    .map(|r| Recipe {
        id: r.id,
        title: r.title,
        description: r.descr,
        serves: r.serves,
        components: vec![],
        image_url: r.image_url,
    })
    .collect();
    Ok(recipes)
}

pub async fn recipes_by_keyword(
    db: &mut sqlx::SqliteConnection,
    keyword: &str,
) -> Result<Vec<Recipe>, sqlx::Error> {
    let recipes = sqlx::query!(
        "SELECT id, title, serves, descr, image_url
        FROM recipes
        WHERE lower(title) LIKE '%' || lower(?) || '%'
            OR lower(descr) LIKE '%' || lower(?) || '%'",
        keyword,
        keyword
    )
    .fetch_all(db)
    .await?
    .into_iter()
    .map(|r| Recipe {
        id: r.id,
        title: r.title,
        description: r.descr,
        serves: r.serves,
        components: vec![],
        image_url: r.image_url,
    })
    .collect();
    Ok(recipes)
}
