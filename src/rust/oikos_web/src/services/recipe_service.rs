use super::Error;
use super::{request::Request, Requests};
use oikos_api::components::schemas::*;
use yew::callback::Callback;
use yew::services::fetch::FetchTask;

#[derive(Default, Debug)]
pub struct RecipeService {
    requests: Requests,
    pub get_all: Request,
}

impl RecipeService {
    pub fn new() -> Self {
        Self {
            requests: Requests::new(),
            get_all: Request::new(),
        }
    }

    pub fn get_recipes2(&mut self, callback: Callback<Result<RecipeList, Error>>) {
        self.get_all.request(
            self.requests
                .get::<RecipeList>("/api/recipes".to_string(), callback),
        );
    }

    pub fn get_recipes(&mut self, callback: Callback<Result<RecipeList, Error>>) -> FetchTask {
        self.requests
            .get::<RecipeList>("/api/recipes".to_string(), callback)
    }

    pub fn get_recipe_by_id(
        &mut self,
        recipe_id: &str,
        callback: Callback<Result<RecipeModel, Error>>,
    ) -> FetchTask {
        self.requests
            .get::<RecipeModel>(format!("/api/recipes/{}", recipe_id), callback)
    }

    pub fn update_recipe_by_id(
        &mut self,
        recipe_id: &str,
        recipe: RecipeModel,
        callback: Callback<Result<RecipeModel, Error>>,
    ) -> FetchTask {
        self.requests.put::<RecipeModel, RecipeModel>(
            format!("/api/recipes/{}", recipe_id),
            recipe,
            callback,
        )
    }

    pub fn add_recipe(
        &mut self,
        recipe: RecipeModel,
        callback: Callback<Result<RecipeModel, Error>>,
    ) -> FetchTask {
        self.requests
            .post::<RecipeModel, RecipeModel>("/api/recipes".to_string(), recipe, callback)
    }

    pub fn _delete_recipe_by_id(
        &mut self,
        recipe_id: &str,
        callback: Callback<Result<(), Error>>,
    ) -> FetchTask {
        self.requests
            ._delete::<()>(format!("/api/recipes/{}", recipe_id), callback)
    }
}
