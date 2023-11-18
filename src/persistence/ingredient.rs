use sqlx::Connection;

use crate::model::{DietGoal, FoodGroup, Ingredient, SpecialDiet};

pub async fn get_ingredient(
    db: &mut sqlx::SqliteConnection,
    id: i64,
) -> Result<Option<Ingredient>, sqlx::Error> {
    sqlx::query!(
        "SELECT id, title, descr
        FROM ingredients
        WHERE id = ?",
        id
    )
    .fetch_optional(db)
    .await
    .map(|r| {
        r.map(|r| Ingredient {
            id: r.id,
            title: r.title,
            description: r.descr,
            ..Default::default()
        })
    })
}

pub async fn get_ingredient_tags(
    db: &mut sqlx::SqliteConnection,
    id: i64,
) -> Result<(Vec<FoodGroup>, Vec<DietGoal>, Vec<SpecialDiet>), sqlx::Error> {
    let mut tx = db.begin().await?;

    let groups = sqlx::query!(
        "SELECT food_group_id
        FROM ingredient_food_groups
        WHERE ingredient_id = ?",
        id
    )
    .fetch_all(&mut *tx)
    .await?
    .into_iter()
    .map(|r| FoodGroup::from(r.food_group_id))
    .collect();

    let goals = sqlx::query!(
        "SELECT diet_goal_id
        FROM ingredient_diet_goals
        WHERE ingredient_id = ?",
        id
    )
    .fetch_all(&mut *tx)
    .await?
    .into_iter()
    .map(|r| DietGoal::from(r.diet_goal_id))
    .collect();

    let diets = sqlx::query!(
        "SELECT special_diet_id
        FROM ingredient_special_diets
        WHERE ingredient_id = ?",
        id
    )
    .fetch_all(&mut *tx)
    .await?
    .into_iter()
    .map(|r| SpecialDiet::from(r.special_diet_id))
    .collect();

    tx.commit().await?;

    Ok((groups, goals, diets))
}

pub async fn get_equivalent_ingredients(
    db: &mut sqlx::SqliteConnection,
    id: i64,
) -> Result<Vec<Ingredient>, sqlx::Error> {
    let results = sqlx::query!(
        "SELECT i.id, i.title, i.descr
        FROM ingredients i
        WHERE i.equivalence = (
            SELECT equivalence
            FROM ingredients
            WHERE id = ?
        )",
        id
    )
    .fetch_all(db)
    .await?
    .into_iter()
    .map(|r| Ingredient {
        id: r.id.unwrap_or(-1), // This is a hack, no idea, why this is an Option in the first place. The column is the primary key
        title: r.title,
        description: r.descr,
        ..Default::default()
    })
    .collect();
    Ok(results)
}
