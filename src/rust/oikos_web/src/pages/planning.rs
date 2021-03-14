use crate::components::Tabs;
use crate::{
    components::Token,
    data::MealPlans,
    date::format_date,
    root::{AppRoute, DataHandle},
    services::{Error, MealPlansService, RecipeService},
};
use oikos_api::components::schemas::{RecipeList, RecipeListItem};
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
    link: ComponentLink<Self>,
    show_done_recipes: bool,
}

pub enum Message {
    ChangeRoute(AppRoute),
    RecipesResponse(Result<RecipeList, Error>),
    MealPlansResponse(Result<MealPlans, Error>),
    CheckRecipe(String, String),
    DeleteRecipe(String, String),
    ShowDoneRecipes(bool),
}

impl<STATE: RouterState> PlanningPageComponent<STATE> {
    fn get_recipes(&mut self) {
        self.recipes_task = Some(
            self.recipes_service
                .get_recipes(self.recipes_response.clone()),
        );
    }

    fn get_meal_plans(&mut self) {
        self.meal_plans_service
            .get_meal_plans(self.link.callback(Message::MealPlansResponse));
    }

    fn update_meal_plans(&mut self, meal_plans: Option<MealPlans>) {
        if let Some(meal_plans) = &meal_plans {
            self.meal_plans_service.update_meal_plans(
                meal_plans.clone(),
                self.link.callback(Message::MealPlansResponse),
            );
        }
    }

    fn get_recipe(&self, recipe_id: String) -> Option<RecipeListItem> {
        let recipes = self.handle.state().recipes.clone().unwrap_or_else(Vec::new);
        recipes
            .iter()
            .find(|recipe| recipe.id == recipe_id)
            .cloned()
    }
}

impl<STATE: RouterState> Component for PlanningPageComponent<STATE> {
    type Message = Message;
    type Properties = DataHandle;
    fn create(handle: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            recipes_service: RecipeService::new(),
            meal_plans_service: MealPlansService::new(),
            router: RouteAgentDispatcher::new(),
            recipes_task: None,
            recipes_response: link.callback(Message::RecipesResponse),
            link,
            show_done_recipes: false,
            handle,
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
            Message::MealPlansResponse(meal_plans) => {
                let meal_plans = self.meal_plans_service.get_meal_plans.response(meal_plans);
                self.handle
                    .reduce(move |state| state.meal_plans = meal_plans);
            }
            Message::ChangeRoute(route) => {
                let route = Route::from(route);
                self.router.send(RouteRequest::ChangeRoute(route));
            }
            Message::CheckRecipe(meal_date, recipe_id) => {
                let mut meal_plans = self.handle.state().meal_plans.clone();
                if let Some(meals_plans_option) = meal_plans.as_mut() {
                    if let Some(meal) = meals_plans_option
                        .iter_mut()
                        .find(|meals| meals.date == meal_date)
                    {
                        if let Some(position) = meal
                            .recipes
                            .iter()
                            .position(|recipe| recipe.id == recipe_id)
                        {
                            if let Some(recipe) = meal.recipes.get_mut(position) {
                                recipe.done = true;
                            }
                        }
                    }
                }
                self.update_meal_plans(meal_plans.clone());
                self.handle.reduce(move |state| {
                    state.meal_plans = meal_plans;
                });
            }
            Message::DeleteRecipe(meal_date, recipe_id) => {
                let mut meal_plans = self.handle.state().meal_plans.clone();
                if let Some(meals_plans_option) = meal_plans.as_mut() {
                    if let Some(meal) = meals_plans_option
                        .iter_mut()
                        .find(|meals| meals.date == meal_date)
                    {
                        if let Some(position) = meal
                            .recipes
                            .iter()
                            .position(|recipe| recipe.id == recipe_id)
                        {
                            meal.recipes.remove(position);
                        }
                    }
                }
                self.update_meal_plans(meal_plans.clone());
                self.handle.reduce(move |state| {
                    state.meal_plans = meal_plans;
                });
            }
            Message::ShowDoneRecipes(value) => {
                self.show_done_recipes = value;
            }
        }
        true
    }

    fn change(&mut self, handle: Self::Properties) -> ShouldRender {
        self.handle.neq_assign(handle)
    }

    fn view(&self) -> Html {
        let meal_plans: Option<MealPlans> = self.handle.state().meal_plans.clone();
        let mut html_view = vec![];

        if let Some(mut meal_plans) = meal_plans {
            meal_plans.sort();
            for meal in meal_plans {
                let mut recipes_counter = 0;
                let mut html_recipes = vec![];
                for meal_recipe in &meal.recipes {
                    if let Some(recipe) = self.get_recipe(meal_recipe.id.clone()) {
                        let recipe_id = recipe.id.clone();
                        let on_read_callback = self.link.callback(move |_| {
                            let recipe_id = recipe_id.clone();
                            Message::ChangeRoute(AppRoute::Recipe(recipe_id))
                        });
                        let recipe_id = recipe.id.clone();
                        let meal_date = meal.date.clone();
                        let on_delete_callback = self.link.callback(move |_| {
                            let recipe_id = recipe_id.clone();
                            Message::DeleteRecipe(meal_date.clone(), recipe_id)
                        });
                        let recipe_id = recipe.id.clone();
                        let meal_date = meal.date.clone();
                        let on_check_callback = self.link.callback(move |_| {
                            let recipe_id = recipe_id.clone();
                            Message::CheckRecipe(meal_date.clone(), recipe_id)
                        });

                        if !meal_recipe.done {
                            recipes_counter += 1;
                            html_recipes.push(html! {

                                <li class="waves-effect with-action">
                                    <div class="valign-wrapper">
                                        <div class="list-elem" onclick=on_read_callback>
                                            <div class="title" >
                                                { voca_rs::case::capitalize(&recipe.name, &true) }
                                            </div>
                                        </div>
                                        <div onclick=on_delete_callback class="action clear">
                                            <i class="material-icons clear">{"clear"}</i>
                                        </div>
                                        <div onclick=on_check_callback class="action check">
                                            <i class="material-icons check">{"check"}</i>
                                        </div>
                                    </div>
                                </li>
                            })
                        } else if self.show_done_recipes {
                            recipes_counter += 1;
                            html_recipes.push(html! {
                                <li class="waves-effect with-action">
                                    <div class="valign-wrapper">
                                        <div class="list-elem" onclick=on_read_callback>
                                            <div class="title" >
                                                { voca_rs::case::capitalize(&recipe.name, &true) }
                                            </div>
                                        </div>
                                        <div class="action check selected">
                                            <i class="material-icons check">{"check"}</i>
                                        </div>
                                    </div>
                                </li>
                            })
                        }
                    }
                }

                if recipes_counter > 0 {
                    let meal_date = meal.clone().date;
                    html_view.push(html! {
                        <>
                            <div class="card horizontal">
                                <div class="card-stacked">
                                    <div class="card-content">
                                        <span class="card-title">{format_date(&meal_date)}</span>
                                    </div>
                                    <ul class="list">
                                        {html_recipes}
                                    </ul>
                                </div>
                            </div>
                        </>
                    })
                }
            }
        };

        let expand = if self.show_done_recipes {
            let callback = self.link.callback(move |_| Message::ShowDoneRecipes(false));
            html! {
                <div class="container-action">
                    <a onclick=callback href="#">{"hide done recipes"}</a>
                </div>
            }
        } else {
            let callback = self.link.callback(move |_| Message::ShowDoneRecipes(true));
            html! {
                <div class="container-action">
                    <a onclick=callback href="#">{"show done recipes"}</a>
                </div>
            }
        };

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
                <Tabs title="Planning"/>
                <div class="planning container">
                    <div class="row">
                        <div class="col s12 m6">
                            {html_view}
                            {expand}
                        </div>
                    </div>
                </div>
            </>
        }
    }
}

pub type PlanningPage = SharedStateComponent<PlanningPageComponent>;
