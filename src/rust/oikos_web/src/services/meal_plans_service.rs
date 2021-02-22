use super::Error;
use super::{request::Request, Requests};
use crate::data::MealPlans;
use yew::callback::Callback;

#[derive(Default, Debug)]
pub struct MealPlansService {
    requests: Requests,
    pub get_meal_plans: Request,
    pub update_meal_plans: Request,
}

impl MealPlansService {
    pub fn new() -> Self {
        Self {
            requests: Requests::new(),
            get_meal_plans: Request::new(),
            update_meal_plans: Request::new(),
        }
    }

    pub fn get_meal_plans(&mut self, callback: Callback<Result<MealPlans, Error>>) {
        self.get_meal_plans.request(
            self.requests
                .get::<MealPlans>("/api/meal_plans".to_string(), callback),
        )
    }

    pub fn update_meal_plans(
        &mut self,
        meal_plans: MealPlans,
        callback: Callback<Result<MealPlans, Error>>,
    ) {
        self.update_meal_plans
            .request(self.requests.put::<MealPlans, MealPlans>(
                "/api/meal_plans".to_string(),
                meal_plans,
                callback,
            ))
    }
}
