#![allow(clippy::let_unit_value)]

mod recipe;

use async_trait::async_trait;
use oikos_api::{models::*, server::OikosApi};

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("map error")]
    RecipeError(#[from] recipe::RecipeError),
}

#[derive(Clone)]
pub struct Server {}

impl Server {
    pub async fn new() -> Self {
        Server {}
    }
}

#[async_trait(?Send)]
impl OikosApi for Server {
    type Error = ServerError;

    async fn get_info(
        &self,
        _parameters: get_info::Parameters,
    ) -> Result<get_info::Success, get_info::Error<Self::Error>> {
        use get_info::*;
        Ok(Success::Status200(Status200 {
            version: Some(oikos_api::VERSION.to_string()),
        }))
    }

    async fn get_recipes(
        &self,
        _parameters: get_recipes::Parameters,
    ) -> Result<get_recipes::Success, get_recipes::Error<Self::Error>> {
        use get_recipes::*;
        match self.get_recipes() {
            Ok(recipes) => Ok(Success::Status200(recipes)),
            Err(err) => Err(Error::Unknown(err.into())),
        }
    }

    async fn add_recipe(
        &self,
        _parameters: add_recipe::Parameters,
        body: add_recipe::Body,
    ) -> Result<add_recipe::Success, add_recipe::Error<Self::Error>> {
        use add_recipe::*;
        match self.add_recipe(&body).await {
            Ok(recipe) => Ok(Success::Status200(recipe)),
            Err(err) => Err(Error::Unknown(err.into())),
        }
    }

    async fn get_recipe_by_id(
        &self,
        get_recipe_by_id::Parameters { recipe_id }: get_recipe_by_id::Parameters,
    ) -> Result<get_recipe_by_id::Success, get_recipe_by_id::Error<Self::Error>> {
        use get_recipe_by_id::*;
        match self.get_recipe_by_id(&recipe_id) {
            Ok(recipe) => Ok(Success::Status200(recipe)),
            Err(err @ recipe::RecipeError::NotFound(_)) => Err(Error::Status404(err.to_string())),
            Err(err) => Err(Error::Unknown(err.into())),
        }
    }

    async fn update_recipe_by_id(
        &self,
        update_recipe_by_id::Parameters { recipe_id }: update_recipe_by_id::Parameters,
        body: update_recipe_by_id::Body,
    ) -> Result<update_recipe_by_id::Success, update_recipe_by_id::Error<Self::Error>> {
        use update_recipe_by_id::*;
        match self.update_recipe_by_id(&recipe_id, &body).await {
            Ok(recipe) => Ok(Success::Status200(recipe)),
            Err(err @ recipe::RecipeError::NotFound(_)) => Err(Error::Status404(err.to_string())),
            Err(err) => Err(Error::Unknown(err.into())),
        }
    }

    async fn delete_recipe_by_id(
        &self,
        delete_recipe_by_id::Parameters { recipe_id }: delete_recipe_by_id::Parameters,
        _body: delete_recipe_by_id::Body,
    ) -> Result<delete_recipe_by_id::Success, delete_recipe_by_id::Error<Self::Error>> {
        use delete_recipe_by_id::*;
        match self.delete_recipe_by_id(&recipe_id) {
            Ok(_) => Ok(Success::Status200(())),
            Err(err @ recipe::RecipeError::NotFound(_)) => Err(Error::Status404(err.to_string())),
            Err(err) => Err(Error::Unknown(err.into())),
        }
    }
}
