#![allow(clippy::let_unit_value)]

mod auth;
mod database;
mod meal_plan;
mod recipe;
mod shopping_list;

use async_trait::async_trait;
use oikos_api::{models::*, server::OikosApi};

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("recipe error")]
    RecipeError(#[from] recipe::RecipeError),
    #[error("auth error")]
    AuthError(#[from] auth::AuthError),
    #[error("meal plans error")]
    MealPlanError(#[from] meal_plan::MealPlanError),
    #[error("shopping list error")]
    ShoppingListError(#[from] shopping_list::ShoppingListError),
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
        get_recipes::Parameters { authorization }: get_recipes::Parameters,
    ) -> Result<get_recipes::Success, get_recipes::Error<Self::Error>> {
        use get_recipes::*;
        match self.get_recipes(&authorization) {
            Ok(recipes) => Ok(Success::Status200(recipes)),
            Err(err) => Err(Error::Unknown(err.into())),
        }
    }

    async fn add_recipe(
        &self,
        add_recipe::Parameters { authorization }: add_recipe::Parameters,
        body: add_recipe::Body,
    ) -> Result<add_recipe::Success, add_recipe::Error<Self::Error>> {
        use add_recipe::*;
        match self.add_recipe(&body, &authorization).await {
            Ok(recipe) => Ok(Success::Status200(recipe)),
            Err(err) => Err(Error::Unknown(err.into())),
        }
    }

    async fn get_recipe_by_id(
        &self,
        get_recipe_by_id::Parameters {
            authorization,
            recipe_id,
        }: get_recipe_by_id::Parameters,
    ) -> Result<get_recipe_by_id::Success, get_recipe_by_id::Error<Self::Error>> {
        use get_recipe_by_id::*;
        match self.get_recipe_by_id(&recipe_id, &authorization) {
            Ok(recipe) => Ok(Success::Status200(recipe)),
            Err(err @ recipe::RecipeError::NotFound(_)) => Err(Error::Status404(err.to_string())),
            Err(err) => Err(Error::Unknown(err.into())),
        }
    }

    async fn update_recipe_by_id(
        &self,
        update_recipe_by_id::Parameters {
            authorization,
            recipe_id,
        }: update_recipe_by_id::Parameters,
        body: update_recipe_by_id::Body,
    ) -> Result<update_recipe_by_id::Success, update_recipe_by_id::Error<Self::Error>> {
        use update_recipe_by_id::*;
        match self
            .update_recipe_by_id(&recipe_id, &body, &authorization)
            .await
        {
            Ok(recipe) => Ok(Success::Status200(recipe)),
            Err(err @ recipe::RecipeError::NotFound(_)) => Err(Error::Status404(err.to_string())),
            Err(err) => Err(Error::Unknown(err.into())),
        }
    }

    async fn delete_recipe_by_id(
        &self,
        delete_recipe_by_id::Parameters {
            authorization,
            recipe_id,
        }: delete_recipe_by_id::Parameters,
        _body: delete_recipe_by_id::Body,
    ) -> Result<delete_recipe_by_id::Success, delete_recipe_by_id::Error<Self::Error>> {
        use delete_recipe_by_id::*;
        match self.delete_recipe_by_id(&recipe_id, &authorization) {
            Ok(_) => Ok(Success::Status200(())),
            Err(err @ recipe::RecipeError::NotFound(_)) => Err(Error::Status404(err.to_string())),
            Err(err) => Err(Error::Unknown(err.into())),
        }
    }

    async fn get_oauth_access_token(
        &self,
        _parameters: get_oauth_access_token::Parameters,
        get_oauth_access_token::Body { code }: get_oauth_access_token::Body,
    ) -> Result<get_oauth_access_token::Success, get_oauth_access_token::Error<Self::Error>> {
        use get_oauth_access_token::*;
        match self.get_oauth_access_token(&code).await {
            Ok(access_token) => Ok(Success::Status200(access_token)),
            Err(err) => Err(Error::Unknown(err.into())),
        }
    }

    async fn get_meal_plans(
        &self,
        get_meal_plans::Parameters { authorization }: get_meal_plans::Parameters,
    ) -> Result<get_meal_plans::Success, get_meal_plans::Error<Self::Error>> {
        use get_meal_plans::*;
        match self.get_meal_plans(&authorization).await {
            Ok(meal_plans) => Ok(Success::Status200(meal_plans)),
            Err(err) => Err(Error::Unknown(err.into())),
        }
    }

    async fn update_meal_plans(
        &self,
        update_meal_plans::Parameters { authorization }: update_meal_plans::Parameters,
        meal_plans: update_meal_plans::Body,
    ) -> Result<update_meal_plans::Success, update_meal_plans::Error<Self::Error>> {
        use update_meal_plans::*;
        match self.update_meal_plans(meal_plans, &authorization).await {
            Ok(meal_plans) => Ok(Success::Status200(meal_plans)),
            Err(err) => Err(Error::Unknown(err.into())),
        }
    }

    async fn get_shopping_list(
        &self,
        get_shopping_list::Parameters { authorization }: get_shopping_list::Parameters,
    ) -> Result<get_shopping_list::Success, get_shopping_list::Error<Self::Error>> {
        use get_shopping_list::*;
        match self.get_shopping_list(&authorization).await {
            Ok(shopping_list) => Ok(Success::Status200(shopping_list)),
            Err(err) => Err(Error::Unknown(err.into())),
        }
    }
}
