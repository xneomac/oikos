use crate::components::Tabs;
use crate::components::Token;
use crate::root::{AppRoute, DataHandle};
use crate::{
    data::{MealPlans, MealPlansItem, MealPlansItemRecipesItem},
    date::next_seven_days,
    services::{Error, MealPlansService, RecipeService},
};
use oikos_api::components::schemas::RecipeList;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::{
    agent::RouteRequest,
    prelude::{Route, RouteAgentDispatcher},
    RouterState,
};
use yew_state::SharedStateComponent;
use yewtil::NeqAssign;

pub struct RecipeListPageComponent<STATE: RouterState = ()> {
    handle: DataHandle,
    link: ComponentLink<Self>,
    router: RouteAgentDispatcher<STATE>,
    meal_plans_service: MealPlansService,
    recipes_service: RecipeService,
    recipe_id: Option<String>,
}

pub enum Message {
    Response(Result<RecipeList, Error>),
    ChangeRoute(AppRoute),
    MealPlansResponse(Result<MealPlans, Error>),
    PlanRecipe(String),
    OpenModal(String),
}

impl<STATE: RouterState> Component for RecipeListPageComponent<STATE> {
    type Message = Message;
    type Properties = DataHandle;
    fn create(handle: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            meal_plans_service: MealPlansService::new(),
            recipes_service: RecipeService::new(),
            link,
            router: RouteAgentDispatcher::new(),
            recipe_id: None,
            handle,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.meal_plans_service
                .get_meal_plans(self.link.callback(Message::MealPlansResponse));
            self.recipes_service
                .get_recipes2(self.link.callback(Message::Response));
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::Response(recipes) => {
                let recipes = self.recipes_service.get_all.response(recipes);
                self.handle.reduce(move |state| state.recipes = recipes);
            }
            Message::MealPlansResponse(meal_plans) => {
                let meal_plans = self.meal_plans_service.get_meal_plans.response(meal_plans);
                self.handle
                    .reduce(move |state| state.meal_plans = meal_plans);
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
                if let Some(meal_plans) = &meal_plans {
                    self.meal_plans_service.update_meal_plans(
                        meal_plans.clone(),
                        self.link.callback(Message::MealPlansResponse),
                    );
                }
                self.handle.reduce(move |state| {
                    state.meal_plans = meal_plans;
                });
                close_modal();
            }
            Message::OpenModal(recipe_id) => {
                self.recipe_id = Some(recipe_id);
                open_modal();
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
                        let on_planning_callback = self.link.callback(move |_| {
                            let recipe_id = recipe_id.clone();
                            Message::OpenModal(recipe_id)
                        });

                        html! {
                            <div class="card horizontal">
                                <div class="card-stacked">
                                    <ul class="list">
                                        <li class="waves-effect with-action">
                                            <div class="valign-wrapper">
                                                <div class="list-elem" onclick=onclick>
                                                    <div class="title" >
                                                        { voca_rs::case::capitalize(&recipe.name, &true) }
                                                    </div>
                                                </div>
                                                <div onclick=on_planning_callback class="action event">
                                                    <i class="material-icons">{"event"}</i>
                                                </div>
                                            </div>
                                        </li>
                                    </ul>
                                </div>
                            </div>
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

        if self.handle.state().meal_plans.is_none() || self.handle.state().recipes.is_none() {
            return html! {
                <>
                    <Token/>
                    <Tabs title="Recettes"/>
                    <div class="loader-page">
                        <div class="preloader-wrapper active">
                            <div class="spinner-layer spinner-red-only">
                            <div class="circle-clipper left">
                                <div class="circle"></div>
                            </div><div class="gap-patch">
                                <div class="circle"></div>
                            </div><div class="circle-clipper right">
                                <div class="circle"></div>
                            </div>
                            </div>
                        </div>
                    </div>
                </>
            };
        }

        html! {
            <>
                <Token/>
                <Tabs title="Recettes"/>

                <div class="planning container">
                    <div class="row">
                        <div class="col s12 m6">
                            {recipe_list}
                        </div>
                    </div>
                </div>

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

#[wasm_bindgen(inline_js = "

        export function close_modal() {
            var elems = document.querySelectorAll('.modal');
            var instances = M.Modal.init(elems);
            var instance = M.Modal.getInstance(elems[0]);
            instance.close();
        }")]
extern "C" {
    fn close_modal();
}
