use oikos_api::components::schemas;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

pub type MealPlans = Vec<MealPlansItem>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MealPlansItem {
    pub date: String,
    pub recipes: Vec<MealPlansItemRecipesItem>,
}

impl PartialOrd for MealPlansItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.date.cmp(&other.date))
    }
}

impl Ord for MealPlansItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.date.cmp(&other.date)
    }
}

impl Eq for MealPlansItem {}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MealPlansItemRecipesItem {
    pub done: bool,
    pub id: String,
    pub servings: Option<f64>,
}

impl From<&schemas::MealPlansItem> for MealPlansItem {
    fn from(data: &schemas::MealPlansItem) -> Self {
        Self {
            date: data.date.clone(),
            recipes: data
                .recipes
                .iter()
                .map(|recipe| recipe.into())
                .collect::<Vec<_>>(),
        }
    }
}

impl From<&schemas::MealPlansItemRecipesItem> for MealPlansItemRecipesItem {
    fn from(data: &schemas::MealPlansItemRecipesItem) -> Self {
        Self {
            done: data.done,
            id: data.id.clone(),
            servings: data.servings,
        }
    }
}
