use crate::components::Tabs;
use crate::{
    components::Token,
    root::{AppRoute, DataHandle},
    services::{Error, MealPlansService, RecipeService},
};
use oikos_api::components::schemas::{MealPlans, RecipeList};
use yew::{prelude::*, services::fetch::FetchTask};
use yew_router::{
    agent::RouteRequest,
    prelude::{Route, RouteAgentDispatcher},
    RouterState,
};
use yew_state::SharedStateComponent;
use yewtil::NeqAssign;

pub struct PlanningPageComponent<STATE: RouterState = ()> {
    handle: DataHandle,
    recipes_service: RecipeService,
    meal_plans_service: MealPlansService,
    router: RouteAgentDispatcher<STATE>,
    recipes_task: Option<FetchTask>,
    recipes_response: Callback<Result<RecipeList, Error>>,
    meal_plans_task: Option<FetchTask>,
    meal_plans_response: Callback<Result<MealPlans, Error>>,
}

pub enum Message {
    ChangeRoute(AppRoute),
    RecipesResponse(Result<RecipeList, Error>),
    MealPlansResponse(Result<MealPlans, Error>),
}

impl<STATE: RouterState> PlanningPageComponent<STATE> {
    fn get_recipes(&mut self) {
        self.recipes_task = Some(
            self.recipes_service
                .get_recipes(self.recipes_response.clone()),
        );
    }

    fn get_meal_plans(&mut self) {
        self.meal_plans_task = Some(
            self.meal_plans_service
                .get_meal_plans(self.meal_plans_response.clone()),
        );
    }
}

impl<STATE: RouterState> Component for PlanningPageComponent<STATE> {
    type Message = Message;
    type Properties = DataHandle;
    fn create(handle: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            handle,
            recipes_service: RecipeService::new(),
            meal_plans_service: MealPlansService::new(),
            router: RouteAgentDispatcher::new(),
            recipes_task: None,
            recipes_response: link.callback(Message::RecipesResponse),
            meal_plans_task: None,
            meal_plans_response: link.callback(Message::MealPlansResponse),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.get_meal_plans();
            self.get_recipes();
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::RecipesResponse(Ok(recipes)) => {
                self.handle
                    .reduce(move |state| state.recipes = Some(recipes));
                self.recipes_task = None;
            }
            Message::RecipesResponse(Err(_)) => {
                self.recipes_task = None;
            }
            Message::MealPlansResponse(Ok(meal_plans)) => {
                self.handle
                    .reduce(move |state| state.meal_plans = Some(meal_plans));
                self.meal_plans_task = None;
            }
            Message::MealPlansResponse(Err(_)) => {
                self.meal_plans_task = None;
            }
            Message::ChangeRoute(route) => {
                let route = Route::from(route);
                self.router.send(RouteRequest::ChangeRoute(route));
            }
        }
        true
    }

    fn change(&mut self, handle: Self::Properties) -> ShouldRender {
        self.handle.neq_assign(handle)
    }

    fn view(&self) -> Html {
        let recipes = self.handle.state().recipes.clone().unwrap_or_else(Vec::new);

        let meal_list = self
            .handle
            .state()
            .meal_plans
            .clone()
            .map(|meal_plans| {
                meal_plans
                    .iter()
                    .map(|meal| {
                        let recipes = meal
                            .recipes
                            .iter()
                            .filter_map(|recipe_meal| {
                                recipes
                                    .iter()
                                    .find(|recipe| recipe.id == recipe_meal.id)
                                    .map(|recipe| {
                                        html! {
                                            <li>{recipe.name.clone()}</li>
                                        }
                                    })
                            })
                            .collect::<Html>();

                        html! {
                            <div class="card">
                                <div class="card-content">
                                    <span class="card-title">{meal.date.clone()}</span>
                                    <ul>
                                        {recipes}
                                    </ul>
                                </div>
                            </div>
                        }
                    })
                    .collect::<Html>()
            })
            .unwrap_or_else(|| html! { <></> });

        html! {
            <>
                <Token/>
                <Tabs title="Planning"/>
                <div class="container">
                    <div class="row">
                        <div class="col s12 m6">
                            {meal_list}
                        </div>
                    </div>
                </div>
            </>
        }
    }
}

pub type PlanningPage = SharedStateComponent<PlanningPageComponent>;
