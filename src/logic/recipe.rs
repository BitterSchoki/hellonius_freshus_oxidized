use crate::model::{DietGoal, FoodGroup, Ingredient, Recipe, RecipeComponent, SpecialDiet, Unit};
use crate::persistence::{ingredient, recipe as recipe_db};

pub async fn get_recipe_with_components(
    db: &mut sqlx::SqliteConnection,
    id: i64,
) -> Result<Option<Recipe>, sqlx::Error> {
    let recipe = recipe_db::get_recipe(db, id).await?;
    let components = recipe_db::get_recipe_components(db, id).await?;
    Ok(recipe.map(|r| Recipe { components, ..r }))
}

pub async fn load_components(
    db: &mut sqlx::SqliteConnection,
    recipe: Recipe,
) -> Result<Recipe, sqlx::Error> {
    let components = recipe_db::get_recipe_components(db, recipe.id).await?;
    Ok(Recipe {
        components,
        ..recipe
    })
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

pub async fn filter_recipes(
    db: &mut sqlx::SqliteConnection,
    food_groups: &[FoodGroup],
    diet_goals: &[DietGoal],
    special_diets: &[SpecialDiet],
) -> Result<Vec<Recipe>, sqlx::Error> {
    let recipes = recipe_db::all_recipes(db).await?;
    let mut filtered = Vec::with_capacity(recipes.len());
    'outer: for recipe in recipes {
        let mut with_components = load_components(db, recipe).await?;
        let mut components = Vec::with_capacity(with_components.components.len());
        for component in with_components.components {
            if let Some(replacement) =
                replace_component(db, component, food_groups, diet_goals, special_diets).await?
            {
                components.push(replacement);
            } else {
                continue 'outer;
            }
        }
        with_components.components = components;
        filtered.push(with_components);
    }
    Ok(filtered)
}

async fn replace_component(
    db: &mut sqlx::SqliteConnection,
    mut component: RecipeComponent,
    food_groups: &[FoodGroup],
    diet_goals: &[DietGoal],
    special_diets: &[SpecialDiet],
) -> Result<Option<RecipeComponent>, sqlx::Error> {
    component.ingredient = crate::logic::ingredient::load_tags(db, component.ingredient).await?;
    let ing = &component.ingredient;
    let mut ok = !food_groups.iter().any(|fg| ing.food_groups.contains(fg));
    ok &= diet_goals.iter().all(|dg| ing.diet_goals.contains(dg));
    ok &= special_diets
        .iter()
        .all(|sd| ing.special_diets.contains(sd));
    if !ok {
        // need to swap
        let replacement = find_replacements(db, &component, food_groups, diet_goals, special_diets)
            .await?
            .pop();
        Ok(replacement)
    } else {
        Ok(Some(component))
    }
}

async fn find_replacements(
    db: &mut sqlx::SqliteConnection,
    component: &RecipeComponent,
    food_groups: &[FoodGroup],
    diet_goals: &[DietGoal],
    special_diets: &[SpecialDiet],
) -> Result<Vec<RecipeComponent>, sqlx::Error> {
    // TODO
    Ok(vec![RecipeComponent {
        ingredient: Ingredient {
            id: -1,
            title: "Replaced".into(),
            description: "".into(),
            ..Default::default()
        },
        amount: 0.0,
        unit: Unit::Grams,
    }])
    // Ok(vec![])
}
