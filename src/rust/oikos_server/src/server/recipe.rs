#![allow(clippy::let_unit_value)]
use super::database::ingredient::ingredient_info;
use oikos_api::models::components::schemas::*;
use serde::{Deserialize, Serialize};
use uniqdb::{
    github::{GithubDb, GithubDbError},
    UniqDb,
};

#[derive(Debug, thiserror::Error)]
pub enum RecipeError {
    #[error("recipe with id `{0}` not found")]
    NotFound(String),
    #[error("access token not found")]
    AccessTokenNotFound(#[from] std::env::VarError),
    #[error("unimplemented")]
    UnimplementedError,
    #[error("unknow error")]
    UnknowError,
    #[error("base64 decode error")]
    DecodeError(#[from] base64::DecodeError),
    #[error("utf8 error")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("serde error")]
    SerdeError(#[from] serde_json::error::Error),
    #[error("invalid data {0}")]
    InvalidDataError(String),
    #[error("github error")]
    GithubDbError(#[from] GithubDbError),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
struct GithubRepo {
    name: String,
    description: Option<String>,
}

impl crate::server::Server {
    pub fn get_recipes(&self, authorization: &str) -> Result<RecipeList, RecipeError> {
        let db = GithubDb::new(authorization, "open-cooking")?;
        let repo_list: Vec<GithubRepo> = db.get_all()?;
        let recipe_list = repo_list
            .iter()
            .map(|recipe| RecipeListItem {
                id: recipe.name.to_string(),
                name: recipe
                    .description
                    .clone()
                    .map_or_else(|| recipe.name.to_string(), |value| value),
            })
            .collect();
        Ok(recipe_list)
    }

    pub async fn add_recipe(
        &self,
        recipe: &RecipeModel,
        authorization: &str,
    ) -> Result<RecipeModel, RecipeError> {
        let db = GithubDb::new(authorization, "open-cooking")?;
        db.create(&recipe.id, &recipe.name, recipe)?;
        Ok(recipe.clone())
    }

    pub fn delete_recipe_by_id(
        &self,
        recipe_id: &str,
        authorization: &str,
    ) -> Result<(), RecipeError> {
        let db = GithubDb::new(authorization, "open-cooking")?;
        <GithubDb as UniqDb<RecipeModel>>::delete(&db, recipe_id)?;
        Ok(())
    }

    pub fn get_recipe_by_id(
        &self,
        recipe_id: &str,
        authorization: &str,
    ) -> Result<RecipeModel, RecipeError> {
        let db = GithubDb::new(authorization, "open-cooking")?;
        let mut recipe: RecipeModel = db.get(recipe_id)?;

        recipe.ingredients.iter_mut().for_each(|ingredient| {
            if ingredient.icon.is_none() {
                if let Some(info) = ingredient_info(&ingredient.name) {
                    ingredient.icon = Some(info.icon.to_string());
                    ingredient.category = Some(info.category.to_string());
                }
            }
        });

        Ok(recipe)
    }

    pub async fn update_recipe_by_id(
        &self,
        recipe_id: &str,
        recipe: &RecipeModel,
        authorization: &str,
    ) -> Result<RecipeModel, RecipeError> {
        let db = GithubDb::new(authorization, "open-cooking")?;
        let updated_recipe = db.update(recipe_id, recipe)?;
        Ok(updated_recipe)
    }
}
