use super::Error;
use super::Requests;
use oikos_api::components::schemas::*;
use yew::callback::Callback;
use yew::services::fetch::FetchTask;

#[derive(Default, Debug)]
pub struct MealPlansService {
    requests: Requests,
}

impl MealPlansService {
    pub fn new() -> Self {
        Self {
            requests: Requests::new(),
        }
    }

    pub fn get_meal_plans(&mut self, callback: Callback<Result<MealPlans, Error>>) -> FetchTask {
        self.requests
            .get::<MealPlans>("/api/meal_plans".to_string(), callback)
    }

    pub fn update_meal_plans(
        &mut self,
        meal_plans: MealPlans,
        callback: Callback<Result<MealPlans, Error>>,
    ) -> FetchTask {
        self.requests.put::<MealPlans, MealPlans>(
            "/api/meal_plans".to_string(),
            meal_plans,
            callback,
        )
    }
}
