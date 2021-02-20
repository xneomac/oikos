use crate::components::Tabs;
use crate::components::Token;
use crate::root::{AppRoute, DataHandle};
use crate::{
    date::next_seven_days,
    services::{Error, MealPlansService, RecipeService},
};
use oikos_api::components::schemas::*;
use wasm_bindgen::prelude::*;
use yew::{prelude::*, services::fetch::FetchTask};
use yew_router::{
    agent::RouteRequest,
    prelude::{Route, RouteAgentDispatcher},
    RouterState,
};
use yew_state::SharedStateComponent;
use yewtil::NeqAssign;

pub struct RecipeListPageComponent<STATE: RouterState = ()> {
    handle: DataHandle,
    recipes_service: RecipeService,
    meal_plans_service: MealPlansService,
    meal_plans_task: Option<FetchTask>,
    meal_plans_response: Callback<Result<MealPlans, Error>>,
    link: ComponentLink<Self>,
    router: RouteAgentDispatcher<STATE>,
    recipes_task: Option<FetchTask>,
    response: Callback<Result<RecipeList, Error>>,
    recipe_id: Option<String>,
}

pub enum Message {
    Response(Result<RecipeList, Error>),
    ChangeRoute(AppRoute),
    MealPlansResponse(Result<MealPlans, Error>),
    PlanRecipe(String),
    OpenModal(String),
}

impl<STATE: RouterState> RecipeListPageComponent<STATE> {
    fn get_recipes(&mut self) {
        self.recipes_task = Some(self.recipes_service.get_recipes(self.response.clone()));
    }

    fn get_meal_plans(&mut self) {
        self.meal_plans_task = Some(
            self.meal_plans_service
                .get_meal_plans(self.meal_plans_response.clone()),
        );
    }

    fn update_meal_plans(&mut self, meal_plans: Option<MealPlans>) {
        if let Some(meal_plans) = meal_plans {
            self.meal_plans_task = Some(
                self.meal_plans_service
                    .update_meal_plans(meal_plans, self.meal_plans_response.clone()),
            );
        }
    }
}

impl<STATE: RouterState> Component for RecipeListPageComponent<STATE> {
    type Message = Message;
    type Properties = DataHandle;
    fn create(handle: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            handle,
            recipes_service: RecipeService::new(),
            meal_plans_service: MealPlansService::new(),
            meal_plans_response: link.callback(Message::MealPlansResponse),
            meal_plans_task: None,
            response: link.callback(Message::Response),
            link,
            router: RouteAgentDispatcher::new(),
            recipes_task: None,
            recipe_id: None,
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
            Message::Response(Ok(recipes)) => {
                self.handle
                    .reduce(move |state| state.recipes = Some(recipes));
                self.recipes_task = None;
            }
            Message::Response(Err(_)) => {
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
            Message::PlanRecipe(meal_date) => {
                let mut meal_plans = self.handle.state().meal_plans.clone();
                if let Some(meals_plans_option) = meal_plans.as_mut() {
                    if let Some(recipe_id) = self.recipe_id.clone() {
                        if let Some(meal) = meals_plans_option
                            .iter_mut()
                            .find(|meals| meals.date == meal_date)
                        {
                            meal.recipes.push(MealPlansItemRecipesItem {
                                done: false,
                                id: recipe_id,
                                servings: None,
                            })
                        } else {
                            meals_plans_option.push(MealPlansItem {
                                date: meal_date,
                                recipes: vec![MealPlansItemRecipesItem {
                                    done: false,
                                    id: recipe_id,
                                    servings: None,
                                }],
                            })
                        }
                    }
                }
                self.update_meal_plans(meal_plans.clone());
                self.handle.reduce(move |state| {
                    state.meal_plans = meal_plans;
                });
            }
            Message::OpenModal(recipe_id) => {
                self.recipe_id = Some(recipe_id);
                unsafe { open_modal() }
            }
        }
        true
    }

    fn change(&mut self, handle: Self::Properties) -> ShouldRender {
        self.handle.neq_assign(handle)
    }

    fn view(&self) -> Html {
        let on_add_callback = self
            .link
            .callback(move |_| Message::ChangeRoute(AppRoute::NewRecipe));

        let recipe_list = self
            .handle
            .state()
            .recipes
            .clone()
            .map(|recipe_list| {
                recipe_list
                    .iter()
                    .map(|recipe| {
                        let recipe_id = recipe.id.clone();
                        let onclick = self.link.callback(move |_| {
                            let recipe_id = recipe_id.clone();
                            Message::ChangeRoute(AppRoute::Recipe(recipe_id))
                        });
                        let recipe_id = recipe.id.clone();
                        let on_planning_callback = self
                            .link
                            .callback(move |_| {
                                let recipe_id = recipe_id.clone();
                                Message::OpenModal(recipe_id)
                            });

                        html! {
                            <li class="waves-effect">
                                <div class="valign-wrapper">
                                    <div class="list-elem" onclick=onclick>
                                        <div class="title" >
                                            { recipe.name.clone() }
                                        </div>
                                    </div>
                                    <i onclick=on_planning_callback class="material-icons ml-auto">{"event"}</i>
                                </div>
                            </li>
                        }
                    })
                    .collect::<Html>()
            })
            .unwrap_or_else(|| html! { <></> });

        let next_date = next_seven_days()
            .iter()
            .map(|(day_code, day_string)| {
                let day_code = day_code.clone();
                let callback = self.link.callback(move |_| {
                    let day_code = day_code.clone();
                    Message::PlanRecipe(day_code)
                });
                html! {
                    <li onclick=callback class="waves-effect">
                        <div class="valign-wrapper">{day_string}</div>
                    </li>
                }
            })
            .collect::<Html>();

        html! {
            <>
                <Token/>
                <Tabs title="Recettes"/>
                <ul class="list">
                    {recipe_list}
                </ul>
                <div class="fixed-action-btn">
                    <a class="btn-floating btn-large red" onclick=on_add_callback>
                        <i class="large material-icons">{"add"}</i>
                    </a>
                </div>
                <div id="modal1" class="modal bottom-sheet">
                    <div class="modal-content">
                        <ul class="list">
                            {next_date}
                        </ul>
                    </div>
                </div>
            </>
        }
    }
}

pub type RecipeListPage = SharedStateComponent<RecipeListPageComponent>;

#[wasm_bindgen(inline_js = "

        export function open_modal() {
            var elems = document.querySelectorAll('.modal');
            var instances = M.Modal.init(elems);
            var instance = M.Modal.getInstance(elems[0]);
            instance.open();
        }")]
extern "C" {
    fn open_modal();
}
