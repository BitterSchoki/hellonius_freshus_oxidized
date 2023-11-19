use crate::model::{Filters, Ingredient, Recipe, RecipeComponent, Unit};
use crate::persistence::ingredient as ingredient_db;
use crate::persistence::recipe as recipe_db;

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
    filters: &Filters,
) -> Result<Vec<Recipe>, sqlx::Error> {
    let recipes = recipe_db::recipes_by_keyword(db, &filters.keyword).await?;
    let mut filtered = Vec::with_capacity(recipes.len());
    'outer: for recipe in recipes {
        let mut with_components = load_components(db, recipe).await?;
        let mut components = Vec::with_capacity(with_components.components.len());
        for component in with_components.components {
            if filters.deadly.contains(&component.ingredient.id) {
                continue 'outer;
            }
            let optional = component.optional;
            if let Some(replacement) = replace_component(db, component.clone(), filters).await? {
                components.push(replacement);
            } else {
                if optional {
                    let removed = RecipeComponent {
                        is_removed: true,
                        ..component
                    };
                    components.push(removed);
                } else {
                    // If a required component can't be replaced, drop the whole recipe
                    continue 'outer;
                }
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
    filters: &Filters,
) -> Result<Option<RecipeComponent>, sqlx::Error> {
    component.ingredient = crate::logic::ingredient::load_tags(db, component.ingredient).await?;
    if !filters.apply(&component.ingredient) {
        Ok(find_replacements(db, &component, filters).await?.pop())
    } else {
        Ok(Some(component))
    }
}

pub async fn find_replacements(
    db: &mut sqlx::SqliteConnection,
    component: &RecipeComponent,
    filters: &Filters,
) -> Result<Vec<RecipeComponent>, sqlx::Error> {
    let equiv = ingredient_db::get_equivalent_ingredients(db, component.ingredient.id).await?;
    let mut equiv_tags = Vec::with_capacity(equiv.len());
    for i in equiv.into_iter() {
        equiv_tags.push(crate::logic::ingredient::load_tags(db, i).await?);
    }

    // Get satisfying ingredients
    let sat = equiv_tags
        .into_iter()
        .filter(|i| filters.apply(i))
        .map(|i| RecipeComponent {
            ingredient: i,
            amount: component.amount,
            unit: component.unit,
            was_replaced: true,
            old_ingredient: Some(component.ingredient.clone()),
            is_removed: false,
            optional: component.optional,
        })
        .collect();

    Ok(sat)
}
