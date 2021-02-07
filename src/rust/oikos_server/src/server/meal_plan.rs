use oikos_api::components::schemas::MealPlans;

use serde::Serialize;
use uniqdb::{
    github::{GithubDb, GithubDbError},
    UniqDb,
};

#[derive(Debug, thiserror::Error)]
pub enum MealPlanError {
    #[error("missing env var")]
    MissingEnvVar(#[from] std::env::VarError),
    #[error("reqwest error")]
    ReqwestError(#[from] reqwest::Error),
    #[error("unknown response")]
    UnknownResponse,
    #[error("github error")]
    GithubDbError(#[from] GithubDbError),
}

#[derive(Serialize)]
pub struct AccessTokenRequest {
    client_id: String,
    client_secret: String,
    code: String,
}

impl crate::server::Server {
    pub async fn get_meal_plans(&self, authorization: &str) -> Result<MealPlans, MealPlanError> {
        let db = GithubDb::new(authorization, "xneomac")?;
        let recipe = db.get("meal_plans")?;
        Ok(recipe)
    }

    pub async fn update_meal_plans(
        &self,
        meal_plan: MealPlans,
        authorization: &str,
    ) -> Result<MealPlans, MealPlanError> {
        let db = GithubDb::new(authorization, "xneomac")?;
        let updated_recipe = db.update("meal_plans", &meal_plan)?;
        Ok(updated_recipe)
    }
}
