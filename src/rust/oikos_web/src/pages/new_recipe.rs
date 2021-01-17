use crate::components::Token;
use crate::root::{AppAnchor, AppRoute};
use crate::services::{Error, RecipeService};
use oikos_api::components::schemas::{
    RecipeIngredientModel, RecipeIngredientModelAmounts, RecipeModel, RecipeModelSteps,
};
use uuid::Uuid;
use yew::{prelude::*, services::fetch::FetchTask};
use yew_router::{
    agent::RouteRequest,
    prelude::{Route, RouteAgentDispatcher},
    RouterState,
};

pub struct NewRecipePage<STATE: RouterState = ()> {
    link: ComponentLink<Self>,
    router: RouteAgentDispatcher<STATE>,
    recipe_service: RecipeService,
    recipe: RecipeModel,
    task: Option<FetchTask>,
    response: Callback<Result<RecipeModel, Error>>,
}

pub enum Message {
    OnAdd,
    OnRecipeAdd(Result<RecipeModel, Error>),
    OnNameChange(String),
    OnIngredientAmountChange(usize, String),
    OnIngredientUnitChange(usize, String),
    OnIngredientNameChange(usize, String),
    OnIngredientDelete(usize),
    OnIngredientAdd,
    OnStepChange(usize, String),
    OnStepDelete(usize),
    OnStepAdd,
}

impl<STATE: RouterState> NewRecipePage<STATE> {
    fn add_recipe(&mut self) {
        self.task = Some(
            self.recipe_service
                .add_recipe(self.recipe.clone(), self.response.clone()),
        );
    }
}

impl<STATE: RouterState> Component for NewRecipePage<STATE> {
    type Message = Message;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            recipe_service: RecipeService::new(),
            router: RouteAgentDispatcher::new(),
            recipe: RecipeModel {
                id: Uuid::new_v4().to_string(),
                ingredients: vec![],
                name: "".to_string(),
                notes: None,
                oven_fan: None,
                oven_temp: None,
                source_authors: None,
                source_book: None,
                source_url: None,
                steps: None,
                yields: None,
            },
            task: None,
            response: link.callback(Message::OnRecipeAdd),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::OnAdd => {
                self.add_recipe();
            }
            Message::OnRecipeAdd(Ok(recipe)) => {
                self.task = None;
                let route = Route::from(AppRoute::Recipe(recipe.id));
                self.router.send(RouteRequest::ChangeRoute(route));
            }
            Message::OnRecipeAdd(Err(_)) => {
                self.task = None;
            }
            Message::OnNameChange(name) => {
                self.recipe.name = name;
            }
            Message::OnIngredientAmountChange(index, amount) => {
                if let Some(ingredient) = self.recipe.ingredients.get_mut(index) {
                    let ingredient_amount = ingredient.amounts.get_mut(0).unwrap();
                    ingredient_amount.amount = amount.parse::<i64>().unwrap();
                }
            }
            Message::OnIngredientUnitChange(index, unit) => {
                if let Some(ingredient) = self.recipe.ingredients.get_mut(index) {
                    let ingredient_amount = ingredient.amounts.get_mut(0).unwrap();
                    ingredient_amount.unit = unit;
                }
            }
            Message::OnIngredientNameChange(index, name) => {
                if let Some(ingredient) = self.recipe.ingredients.get_mut(index) {
                    ingredient.name = name;
                }
            }
            Message::OnIngredientAdd => {
                self.recipe.ingredients.push(RecipeIngredientModel {
                    amounts: vec![RecipeIngredientModelAmounts {
                        amount: 1,
                        unit: "".to_string(),
                    }],
                    name: "".to_string(),
                    notes: None,
                    processing: None,
                    substitutions: None,
                    usda_num: None,
                });
            }
            Message::OnIngredientDelete(index) => {
                self.recipe.ingredients.remove(index);
            }
            Message::OnStepChange(index, step) => {
                if let Some(steps) = self.recipe.steps.as_mut() {
                    if let Some(selected_step) = steps.get_mut(index) {
                        selected_step.step = step;
                    }
                }
            }
            Message::OnStepAdd => {
                if let Some(steps) = self.recipe.steps.as_mut() {
                    steps.push(RecipeModelSteps {
                        haccp: None,
                        notes: None,
                        step: "".to_string(),
                    });
                }
            }
            Message::OnStepDelete(index) => {
                if let Some(steps) = self.recipe.steps.as_mut() {
                    steps.remove(index);
                }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let on_save = self.link.callback(|_| Message::OnAdd);
        let on_name_change_callback = self
            .link
            .callback(|e: InputData| Message::OnNameChange(e.value));
        let on_ingredient_add_callback = self.link.callback(|_| Message::OnIngredientAdd);
        let on_step_add_callback = self.link.callback(|_| Message::OnStepAdd);

        let ingredients = self.recipe.clone()
            .ingredients
            .iter()
            .enumerate()
            .map(|(index, ingredient)| {
                let amount = ingredient.amounts.get(0).unwrap();
                let on_ingredient_amount_change_callback = self
                    .link
                    .callback(move |e: InputData| Message::OnIngredientAmountChange(index, e.value));
                let on_ingredient_unit_change_callback = self
                    .link
                    .callback(move |e: InputData| Message::OnIngredientUnitChange(index, e.value));
                let on_ingredient_name_change_callback = self
                    .link
                    .callback(move |e: InputData| Message::OnIngredientNameChange(index, e.value));
                let on_ingredient_delete_callback = self
                    .link
                    .callback(move |_| Message::OnIngredientDelete(index));

                html! {
                    <div class="row">
                        <div class="input-field col s2">
                            <input value={amount.amount.clone()} oninput={on_ingredient_amount_change_callback} id="quantity" type="text" class="validate"/>
                        </div>
                        <div class="input-field col s2">
                            <input value={amount.unit.clone()} oninput={on_ingredient_unit_change_callback} id="unit" type="text" class="validate"/>
                        </div>
                        <div class="input-field col s6">
                            <input value={ingredient.name.clone()} oninput={on_ingredient_name_change_callback} id="name" type="text" class="validate"/>
                        </div>
                        <div class="input-field col s2">
                            <a onclick={on_ingredient_delete_callback} class="waves-effect waves-teal btn-flat"><i class="material-icons">{"delete"}</i></a>
                        </div>
                    </div>
                }
            })
            .collect::<Html>();
        let steps = match self.recipe.clone()
            .steps {
            Some(steps) => steps.iter().enumerate()
            .map(|(index, step)| {
                let on_step_change_callback = self
                    .link
                    .callback(move |e: InputData| Message::OnStepChange(index, e.value));
                let on_step_delete_callback = self
                    .link
                    .callback(move |_| Message::OnStepDelete(index));

                html! {
                    <div class="row">
                        <div class="input-field col s10">
                            <textarea oninput={on_step_change_callback} value={step.step.clone()} class="materialize-textarea"></textarea>
                        </div>
                        <div class="input-field col s2">
                            <a onclick={on_step_delete_callback} class="waves-effect waves-teal btn-flat"><i class="material-icons">{"delete"}</i></a>
                        </div>
                    </div>
                }
            }).collect::<Html>(),
            None => html!{}
        };

        html! {
            <>
                <Token/>
                <div class="navbar-fixed">
                    <nav>
                        <div class="nav-wrapper">
                            <ul class="left">
                                <AppAnchor route=AppRoute::RecipeList>
                                    <i class="material-icons">{"chevron_left"}</i>
                                </AppAnchor>
                            </ul>

                            <ul class="right">
                                <li><a onclick={on_save}><i class="material-icons">{"save"}</i></a></li>
                            </ul>
                        </div>
                    </nav>
                </div>
                <div class="container">
                    <div class="section">
                        <div class="row">
                            <form class="col s12">
                                <div class="row">
                                    <div class="input-field col s12">
                                        <input value={self.recipe.name.clone()} oninput={on_name_change_callback} type="text" class="validate"/>
                                    </div>
                                </div>
                            </form>
                        </div>
                    </div>
                    <div class="divider"></div>
                    <div class="section">
                        <h5>{"Ingrédients"}</h5>
                        <div class="row">
                            <form class="col s12">
                                {ingredients}
                                <div class="row">
                                    <div class="input-field col s12">
                                        <a onclick={on_ingredient_add_callback} class="waves-effect waves-teal btn-flat"><i class="material-icons">{"add"}</i>{"ajouter"}</a>
                                    </div>
                                </div>
                            </form>
                        </div>
                    </div>
                    <div class="divider"></div>
                    <div class="section">
                        <h5>{"Instructions"}</h5>
                        <div class="row">
                            <form class="col s12">
                                {steps}
                                <div class="row">
                                    <div class="input-field col s12">
                                        <a onclick={on_step_add_callback} class="waves-effect waves-teal btn-flat"><i class="material-icons">{"add"}</i>{"ajouter"}</a>
                                    </div>
                                </div>
                            </form>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}
