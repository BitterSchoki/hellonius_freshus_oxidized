use rocket::serde::json::Json;
use rocket_db_pools::sqlx;
use rocket_db_pools::Connection;

use crate::db;

use super::Ingredient;
use super::Recipe;
use super::RecipeComponent;

#[get("/<id>")]
pub async fn read_recipe(mut db: Connection<db::Recipes>, id: i64) -> Option<Json<Recipe>> {
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
    .fetch_all(&mut **db)
    .await
    .ok()?;
    let mut it = results.into_iter();
    let f = it.nth(0)?;
    let components_res: Result<_, _> = it
        .map(|r| -> Result<_, anyhow::Error> {
            Ok(RecipeComponent {
                ingredient: Ingredient {
                    title: r.i_title,
                    description: r.i_descr,
                },
                amount: r.amount,
                unit: r.unit[..].try_into()?,
            })
        })
        .collect();
    let first_comp = RecipeComponent {
        ingredient: Ingredient {
            title: f.i_title,
            description: f.i_descr,
        },
        amount: f.amount,
        unit: f.unit[..].try_into().ok()?,
    };
    let mut components: Vec<_> = components_res.ok()?;
    components.insert(0, first_comp);
    let recipe = Recipe {
        title: f.r_title,
        description: f.r_descr,
        serves: f.serves,
        components: components,
    };
    Some(recipe.into())
}
