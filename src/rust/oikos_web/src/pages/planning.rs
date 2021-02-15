use crate::components::Tabs;
use crate::{
    components::Token,
    date::format_date,
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
    link: ComponentLink<Self>,
}

pub enum Message {
    ChangeRoute(AppRoute),
    RecipesResponse(Result<RecipeList, Error>),
    MealPlansResponse(Result<MealPlans, Error>),
    CheckRecipe(String),
    DeleteRecipe(String),
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
            link,
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
            Message::CheckRecipe(recipe_id) => {}
            Message::DeleteRecipe(recipe_id) => {}
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
                        let mut recipes_counter = 0;
                        let recipes = meal
                            .recipes
                            .iter()
                            .filter(|recipe_meal| !recipe_meal.done)
                            .filter_map(|recipe_meal| {
                                recipes
                                    .iter()
                                    .find(|recipe| recipe.id == recipe_meal.id)
                                    .map(|recipe| {
                                        recipes_counter += 1;
                                        let recipe_id = recipe.id.clone();
                                        let on_read_callback = self.link.callback(move |_| {
                                            let recipe_id = recipe_id.clone();
                                            Message::ChangeRoute(AppRoute::Recipe(recipe_id))
                                        });
                                        let recipe_id = recipe.id.clone();
                                        let on_delete_callback = self.link.callback(move |_| {
                                            let recipe_id = recipe_id.clone();
                                            Message::CheckRecipe(recipe_id)
                                        });
                                        let recipe_id = recipe.id.clone();
                                        let on_check_callback = self.link.callback(move |_| {
                                            let recipe_id = recipe_id.clone();
                                            Message::DeleteRecipe(recipe_id)
                                        });

                                        html! {
                                            <div class="card horizontal">
                                                <div class="card-stacked">
                                                    <div class="card-content">
                                                        <span class="card-title">{recipe.name.clone()}</span>
                                                    </div>
                                                    <div class="card-action">
                                                        <a onclick=on_read_callback href="#">{"consulter"}</a>
                                                        <a onclick=on_delete_callback href="#">{"supprimer"}</a>
                                                        <a onclick=on_check_callback href="#">{"valider"}</a>
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    })
                            })
                            .collect::<Html>();

                        if recipes_counter > 0 {
                            html! {
                                <>
                                    <h5>{format_date(&meal.date)}</h5>
                                    {recipes}
                                </>
                            }
                        } else {
                            html! {}
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
