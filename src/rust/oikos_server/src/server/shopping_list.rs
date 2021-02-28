use oikos_api::components::schemas::{
    MealPlans, MealPlansItem, MealPlansItemRecipesItem, RecipeModel, ShoppingList,
    ShoppingListCategory, ShoppingListItem,
};

use super::{meal_plan::MealPlanError, recipe::RecipeError};
use serde::Serialize;
use std::collections::HashMap;
use uniqdb::{
    github::{GithubDb, GithubDbError},
    UniqDb,
};

#[derive(Debug, thiserror::Error)]
pub enum ShoppingListError {
    #[error("unknown response")]
    UnknownResponse,
    #[error("github error")]
    GithubDbError(#[from] GithubDbError),
    #[error("meal plans error")]
    MealPlansError(#[from] MealPlanError),
    #[error("recipes error")]
    RecipesError(#[from] RecipeError),
}

impl crate::server::Server {
    pub async fn get_shopping_list(
        &self,
        authorization: &str,
    ) -> Result<ShoppingList, ShoppingListError> {
        let meal_plans: MealPlans = self.get_meal_plans(authorization).await?;
        // let mut shopping_list = ShoppingList { items: vec![] };

        let mut shopping_list = ShoppingList { items: vec![] };

        meal_plans
            .iter()
            .map(|meal_plan_item| {
                meal_plan_item
                    .recipes
                    .iter()
                    .filter(|meal_plan_item_recipe| !meal_plan_item_recipe.done)
                    .map(|meal_plan_item_recipe| {
                        let recipe: RecipeModel =
                            self.get_recipe_by_id(&meal_plan_item_recipe.id, authorization)?;
                        recipe.ingredients.iter().for_each(|ingredient| {
                            let category = ingredient
                                .category
                                .clone()
                                .unwrap_or_else(|| "Sans catégorie".to_string());
                            let shopping_list_item = ShoppingListItem {
                                amount: "0".to_string(),
                                ingredient: ingredient.name.clone(),
                                unit: "".to_string(),
                                icon: ingredient.icon.clone().unwrap_or_else(|| "".to_string()),
                            };

                            let mut shopping_list_category = shopping_list
                                .items
                                .iter_mut()
                                .find(|item| item.name == category);

                            if let Some(shopping_list_category) = &mut shopping_list_category {
                                let is_present = shopping_list_category
                                    .items
                                    .iter()
                                    .any(|item| item.ingredient == ingredient.name);
                                if !is_present {
                                    shopping_list_category.items.push(shopping_list_item);
                                }
                            } else {
                                shopping_list.items.push(ShoppingListCategory {
                                    name: category,
                                    items: vec![shopping_list_item],
                                })
                            }
                        });
                        Ok(())
                    })
                    .collect::<Result<(), ShoppingListError>>()
            })
            .collect::<Result<(), ShoppingListError>>()?;

        Ok(shopping_list)
    }
}
