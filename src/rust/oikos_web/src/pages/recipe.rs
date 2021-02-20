use crate::components::Token;
use crate::root::{AppAnchor, AppRoute, DataHandle, DataState};
use crate::{
    date::next_seven_days,
    services::{Error, MealPlansService, RecipeService},
};
use oikos_api::components::schemas::*;
use yew::{prelude::*, services::fetch::FetchTask, Properties};
use yew_state::{SharedHandle, SharedState, SharedStateComponent};

pub struct RecipePageComponent {
    link: ComponentLink<Self>,
    recipe_service: RecipeService,
    recipe: Option<RecipeModel>,
    task: Option<FetchTask>,
    response: Callback<Result<RecipeModel, Error>>,
    props: Props,
    edit_mode: bool,
    meal_plans_service: MealPlansService,
    meal_plans_task: Option<FetchTask>,
    meal_plans_response: Callback<Result<MealPlans, Error>>,
}

pub enum Message {
    OnRecipeFetch(Result<RecipeModel, Error>),
    OnSave,
    OnNameChange(String),
    OnIngredientAmountChange(usize, String),
    OnIngredientUnitChange(usize, String),
    OnIngredientNameChange(usize, String),
    OnIngredientDelete(usize),
    OnIngredientAdd,
    OnStepChange(usize, String),
    OnStepDelete(usize),
    OnStepAdd,
    OnSourceUrlAdd,
    OnSourceUrlChange(String),
    OnSourceUrlDelete,
    OnQuantityAdd,
    OnQuantityAmountChange(String),
    OnQuantityUnitChange(String),
    OnQuantityDelete,
    OnEditMode,
    OnCancel,
    PlanRecipe(String),
    MealPlansResponse(Result<MealPlans, Error>),
    OpenModal,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub recipe_id: String,
    #[prop_or_default]
    handle: SharedHandle<DataState>,
}

impl SharedState for Props {
    type Handle = SharedHandle<DataState>;

    fn handle(&mut self) -> &mut Self::Handle {
        &mut self.handle
    }
}

impl RecipePageComponent {
    fn get_recipe(&mut self) {
        self.task = Some(
            self.recipe_service
                .get_recipe_by_id(&self.props.recipe_id, self.response.clone()),
        );
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

    fn get_ingredients(&self, recipe: &RecipeModel) -> Html {
        let ingredients = recipe
            .ingredients
            .iter()
            .map(|ingredient| {
                let ingredient_amount = if let Some(amount) = ingredient.amount {
                    format!("{} ", amount)
                } else {
                    "".to_string()
                };

                let ingredient_unit = if let Some(unit) = &ingredient.unit {
                    format!("{} ", unit)
                } else {
                    "".to_string()
                };

                let value = format!(
                    "• {}{}{}",
                    ingredient_amount,
                    ingredient_unit,
                    ingredient.name.clone()
                );
                html! {
                    <li>
                        {value}
                    </li>
                }
            })
            .collect::<Html>();

        html! {
            <ul>
                {ingredients}
            </ul>
        }
    }

    fn get_ingredients_edit(&self, recipe: &RecipeModel) -> Html {
        let ingredients = recipe
            .ingredients
            .iter()
            .enumerate()
            .map(|(index, ingredient)| {
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

                let ingredient_amount = if let Some(amount) = ingredient.amount {
                    format!("{} ", amount)
                } else {
                    "".to_string()
                };

                let ingredient_unit = if let Some(unit) = &ingredient.unit {
                    format!("{} ", unit)
                } else {
                    "".to_string()
                };

                html! {
                    <div class="row">
                        <div class="input-field col s2">
                            <input disabled={!self.edit_mode} value={ingredient_amount} oninput={on_ingredient_amount_change_callback} id="quantity" type="text" class="validate"/>
                        </div>
                        <div class="input-field col s2">
                            <input disabled={!self.edit_mode} value={ingredient_unit} oninput={on_ingredient_unit_change_callback} id="unit" type="text" class="validate"/>
                        </div>
                        <div class="input-field col s6">
                            <input disabled={!self.edit_mode} value={ingredient.name.clone()} oninput={on_ingredient_name_change_callback} id="name" type="text" class="validate"/>
                        </div>
                        <div class="input-field col s2">
                            <button onclick={on_ingredient_delete_callback} class="btn waves-effect waves-light" name="action">
                                <i class="material-icons left">{"delete"}</i>
                            </button>
                        </div>
                    </div>
                }
            })
            .collect::<Html>();
        let on_ingredient_add_callback = self.link.callback(|_| Message::OnIngredientAdd);

        html! {
            <>
                <form class="col s12">
                    {ingredients}
                </form>
                <div class="row">
                    <div class="input-field col s12">
                        <button onclick={on_ingredient_add_callback} class="btn waves-effect waves-light" name="action">
                            {"ingrédient"}
                            <i class="material-icons left">{"add"}</i>
                        </button>
                    </div>
                </div>
            </>
        }
    }

    fn get_source_url(&self, recipe: &RecipeModel) -> Html {
        match &recipe.source_url {
            Some(source_url) => html! {
                <div class="row">
                    {source_url}
                </div>
            },
            None => html! {},
        }
    }

    fn get_source_url_edit(&self, recipe: &RecipeModel) -> Html {
        let source_url = match &recipe.source_url {
            Some(source_url) => {
                let on_source_url_change_callback = self
                    .link
                    .callback(|e: InputData| Message::OnSourceUrlChange(e.value));
                let on_source_url_delete_callback =
                    self.link.callback(|_| Message::OnSourceUrlDelete);
                let delete_source = html! {
                    <div class="input-field col s2">
                        <button onclick={on_source_url_delete_callback} class="btn waves-effect waves-light" name="action">
                            <i class="material-icons left">{"delete"}</i>
                        </button>
                    </div>
                };
                html! {
                    <div class="row">
                        <div class="input-field col s10">
                            <input disabled={!self.edit_mode} oninput={on_source_url_change_callback} value={source_url} type="text" class="validate"/>
                        </div>
                        {delete_source}
                    </div>
                }
            }
            None => {
                let on_source_url_add_callback = self.link.callback(|_| Message::OnSourceUrlAdd);
                html! {
                    <div class="row">
                        <div class="input-field col s12">
                            <button onclick={on_source_url_add_callback} class="btn waves-effect waves-light" name="action">
                                {"source"}
                                <i class="material-icons left">{"add"}</i>
                            </button>
                        </div>
                    </div>
                }
            }
        };
        html! {
            <form class="col s12">
                {source_url}
            </form>
        }
    }

    fn get_instructions_edit(&self, recipe: &RecipeModel) -> Html {
        let on_step_add_callback = self.link.callback(|_| Message::OnStepAdd);
        let steps = match &recipe
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
                                <button onclick={on_step_delete_callback} class="btn waves-effect waves-light" name="action">
                                    <i class="material-icons left">{"delete"}</i>
                                </button>
                            </div>
                        </div>
                    }
                }).collect::<Html>(),
                None => html!{}
            };
        html! {
            <>
                <form class="col s12">
                    {steps}
                </form>
                <div class="row">
                    <div class="input-field col s12">
                        <button onclick={on_step_add_callback} class="btn waves-effect waves-light">
                            {"instruction"}
                            <i class="material-icons left">{"add"}</i>
                        </button>
                    </div>
                </div>
            </>
        }
    }

    fn get_instructions(&self, recipe: &RecipeModel) -> Html {
        let steps = match &recipe.steps {
            Some(steps) => steps
                .iter()
                .enumerate()
                .map(|(index, step)| {
                    let step_index = format!("{}) ", index + 1);
                    html! {
                        <p>
                            {step_index}
                            {step.step.clone()}
                        </p>
                    }
                })
                .collect::<Html>(),
            None => html! {},
        };
        html! {
            <div>
                {steps}
            </div>
        }
    }

    fn get_header(&self, recipe: &RecipeModel) -> Html {
        let quantity = match &recipe.quantity {
            Some(quantity) => {
                let value = format!("{} {}", quantity.amount.clone(), quantity.unit.clone());
                html! {
                    <h6>{value}</h6>
                }
            }
            None => html! {},
        };
        html! {
            <div>
                <h4>{recipe.name.clone()}</h4>
                {quantity}
            </div>
        }
    }

    fn get_header_edit(&self, recipe: &RecipeModel) -> Html {
        let on_name_change_callback = self
            .link
            .callback(|e: InputData| Message::OnNameChange(e.value));
        let quantity = match &recipe.quantity {
            Some(quantity) => {
                let on_quantity_amount_change_callback = self
                    .link
                    .callback(|e: InputData| Message::OnQuantityAmountChange(e.value));
                let on_quantity_unit_change_callback = self
                    .link
                    .callback(|e: InputData| Message::OnQuantityUnitChange(e.value));
                let on_quantity_delete_callback = self.link.callback(|_| Message::OnQuantityDelete);

                html! {
                    <div class="row">
                        <div class="input-field col s3">
                            <input oninput={on_quantity_amount_change_callback} value={quantity.amount.clone()} type="text" class="validate"/>
                        </div>
                        <div class="input-field col s7">
                            <input oninput={on_quantity_unit_change_callback} value={quantity.unit.clone()} type="text" class="validate"/>
                        </div>
                        <div class="input-field col s2">
                            <button onclick={on_quantity_delete_callback} class="btn waves-effect waves-light" name="action">
                                <i class="material-icons left">{"delete"}</i>
                            </button>
                        </div>
                    </div>
                }
            }
            None => {
                let on_quantity_add_callback = self.link.callback(|_| Message::OnQuantityAdd);

                html! {
                    <div class="row">
                        <div class="input-field col s12">
                            <button onclick={on_quantity_add_callback} class="btn waves-effect waves-light" name="action">
                                {"quantité"}
                                <i class="material-icons left">{"add"}</i>
                            </button>
                        </div>
                    </div>
                }
            }
        };

        html! {
            <form class="col s12">
                <div class="row">
                    <div class="input-field col s12">
                        <input disabled={!self.edit_mode} value={recipe.name.clone()} oninput={on_name_change_callback} type="text" class="validate"/>
                    </div>
                </div>
                {quantity}
            </form>
        }
    }

    fn get_menu(&self, _recipe: &RecipeModel) -> Html {
        let callback = self.link.callback(|_| Message::OnEditMode);
        html! {
            <li>
                <a onclick={callback}><i class="material-icons">{"edit"}</i></a>
            </li>
        }
    }

    fn get_edit_menu(&self, _recipe: &RecipeModel) -> Html {
        let on_save_callback = self.link.callback(|_| Message::OnSave);
        let on_cancel_edit_mode_callback = self.link.callback(|_| Message::OnCancel);
        html! {
            <li>
                <a onclick={on_save_callback}>
                    <i class="material-icons">{"save"}</i>
                </a>
                <a onclick=on_cancel_edit_mode_callback>
                    <i class="material-icons">{"close"}</i>
                </a>

            </li>
        }
    }

    fn get_fab(&self, _recipe: &RecipeModel) -> Html {
        let callback = self.link.callback(move |_| Message::OpenModal);
        html! {
            <a class="btn-floating btn-large red" onclick=callback>
                <i class="large material-icons">{"event"}</i>
            </a>
        }
    }

    fn get_fab_edit(&self, _recipe: &RecipeModel) -> Html {
        html! {}
    }
}

impl Component for RecipePageComponent {
    type Message = Message;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            recipe_service: RecipeService::new(),
            response: link.callback(Message::OnRecipeFetch),
            recipe: None,
            task: None,
            meal_plans_service: MealPlansService::new(),
            meal_plans_response: link.callback(Message::MealPlansResponse),
            meal_plans_task: None,
            props,
            link,
            edit_mode: false,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.get_recipe();
            self.get_meal_plans();
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::OnRecipeFetch(Ok(recipe)) => {
                self.recipe = Some(recipe);
                self.task = None;
            }
            Message::OnRecipeFetch(Err(_)) => {
                self.task = None;
            }
            Message::OnSave => {
                if let Some(recipe) = self.recipe.clone() {
                    self.task = Some(self.recipe_service.update_recipe_by_id(
                        &self.props.recipe_id.clone(),
                        recipe,
                        self.response.clone(),
                    ));
                }
            }
            Message::OnNameChange(name) => {
                if let Some(recipe) = self.recipe.as_mut() {
                    recipe.name = name;
                }
            }
            Message::OnIngredientAmountChange(index, amount) => {
                if let Some(recipe) = self.recipe.as_mut() {
                    if let Some(ingredient) = recipe.ingredients.get_mut(index) {
                        ingredient.amount = amount.parse::<f64>().map_or_else(|_err| None, Some);
                    }
                }
            }
            Message::OnIngredientUnitChange(index, unit) => {
                if let Some(recipe) = self.recipe.as_mut() {
                    if let Some(ingredient) = recipe.ingredients.get_mut(index) {
                        ingredient.unit = Some(unit);
                    }
                }
            }
            Message::OnIngredientNameChange(index, name) => {
                if let Some(recipe) = self.recipe.as_mut() {
                    if let Some(ingredient) = recipe.ingredients.get_mut(index) {
                        ingredient.name = name;
                    }
                }
            }
            Message::OnIngredientAdd => {
                if let Some(recipe) = self.recipe.as_mut() {
                    recipe.ingredients.push(RecipeIngredientModel {
                        amount: None,
                        unit: None,
                        name: "".to_string(),
                        notes: None,
                        processing: None,
                        substitutions: None,
                        usda_num: None,
                    });
                }
            }
            Message::OnIngredientDelete(index) => {
                if let Some(recipe) = self.recipe.as_mut() {
                    recipe.ingredients.remove(index);
                }
            }
            Message::OnStepChange(index, step) => {
                if let Some(recipe) = self.recipe.as_mut() {
                    if let Some(steps) = recipe.steps.as_mut() {
                        if let Some(selected_step) = steps.get_mut(index) {
                            selected_step.step = step;
                        }
                    }
                }
            }
            Message::OnStepAdd => {
                if let Some(recipe) = self.recipe.as_mut() {
                    if let Some(steps) = recipe.steps.as_mut() {
                        steps.push(RecipeModelSteps {
                            haccp: None,
                            notes: None,
                            step: "".to_string(),
                        });
                    } else {
                        recipe.steps = Some(vec![RecipeModelSteps {
                            haccp: None,
                            notes: None,
                            step: "".to_string(),
                        }]);
                    }
                }
            }
            Message::OnStepDelete(index) => {
                if let Some(recipe) = self.recipe.as_mut() {
                    if let Some(steps) = recipe.steps.as_mut() {
                        steps.remove(index);
                    }
                }
            }
            Message::OnSourceUrlAdd => {
                if let Some(recipe) = self.recipe.as_mut() {
                    recipe.source_url = Some("".to_string())
                }
            }
            Message::OnSourceUrlChange(source_url) => {
                if let Some(recipe) = self.recipe.as_mut() {
                    recipe.source_url = Some(source_url)
                }
            }
            Message::OnSourceUrlDelete => {
                if let Some(recipe) = self.recipe.as_mut() {
                    recipe.source_url = None
                }
            }
            Message::OnQuantityAdd => {
                if let Some(recipe) = self.recipe.as_mut() {
                    recipe.quantity = Some(RecipeModelQuantity {
                        amount: 1.0,
                        unit: "personne".to_string(),
                    })
                }
            }
            Message::OnQuantityAmountChange(quantity_amount) => {
                if let Some(recipe) = self.recipe.as_mut() {
                    if let Some(quantity) = recipe.quantity.as_mut() {
                        quantity.amount = quantity_amount.parse::<f64>().unwrap_or(1.0);
                    }
                }
            }
            Message::OnQuantityUnitChange(quantity_unit) => {
                if let Some(recipe) = self.recipe.as_mut() {
                    if let Some(quantity) = recipe.quantity.as_mut() {
                        quantity.unit = quantity_unit;
                    }
                }
            }
            Message::OnQuantityDelete => {
                if let Some(recipe) = self.recipe.as_mut() {
                    recipe.quantity = None
                }
            }
            Message::OnEditMode => {
                self.edit_mode = true;
            }
            Message::OnCancel => {
                self.edit_mode = false;
            }
            Message::PlanRecipe(meal_date) => {
                let mut meal_plans = self.props.handle().state().meal_plans.clone();
                if let Some(meals_plans_option) = meal_plans.as_mut() {
                    if let Some(meal) = meals_plans_option
                        .iter_mut()
                        .find(|meals| meals.date == meal_date)
                    {
                        meal.recipes.push(MealPlansItemRecipesItem {
                            done: false,
                            id: self.props.recipe_id.clone(),
                            servings: None,
                        })
                    } else {
                        meals_plans_option.push(MealPlansItem {
                            date: meal_date,
                            recipes: vec![MealPlansItemRecipesItem {
                                done: false,
                                id: self.props.recipe_id.clone(),
                                servings: None,
                            }],
                        })
                    }
                }
                self.update_meal_plans(meal_plans.clone());
                self.props.handle().reduce(move |state| {
                    state.meal_plans = meal_plans;
                });
                unsafe { close_modal() }
            }
            Message::MealPlansResponse(Ok(meal_plans)) => {
                self.props
                    .handle()
                    .reduce(move |state| state.meal_plans = Some(meal_plans));
                self.meal_plans_task = None;
            }
            Message::MealPlansResponse(Err(_)) => {
                self.meal_plans_task = None;
            }
            Message::OpenModal => unsafe { open_modal() },
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        false
    }

    fn view(&self) -> Html {
        match self.recipe.clone() {
            Some(recipe) => {
                let (header, ingredients, instructions, source_url, fab_action, menu_action) =
                    if self.edit_mode {
                        (
                            self.get_header_edit(&recipe),
                            self.get_ingredients_edit(&recipe),
                            self.get_instructions_edit(&recipe),
                            self.get_source_url_edit(&recipe),
                            self.get_fab_edit(&recipe),
                            self.get_edit_menu(&recipe),
                        )
                    } else {
                        (
                            self.get_header(&recipe),
                            self.get_ingredients(&recipe),
                            self.get_instructions(&recipe),
                            self.get_source_url(&recipe),
                            self.get_fab(&recipe),
                            self.get_menu(&recipe),
                        )
                    };

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
                        <div class="navbar-fixed">
                            <nav>
                                <div class="nav-wrapper">
                                    <ul class="left">
                                        <AppAnchor route=AppRoute::RecipeList>
                                            <i class="material-icons">{"chevron_left"}</i>
                                        </AppAnchor>
                                    </ul>

                                    <ul class="right">
                                        {menu_action}
                                    </ul>
                                </div>
                            </nav>
                        </div>

                        <div class="fixed-action-btn">
                            {fab_action}
                        </div>

                        <div id="modal1" class="modal bottom-sheet">
                            <div class="modal-content">
                                <ul class="list">
                                    {next_date}
                                </ul>
                            </div>
                        </div>

                        <div class="container">
                            <div class="section">
                                <div class="row">
                                    {header}
                                </div>
                            </div>
                            <div class="divider"></div>
                            <div class="section">
                                <h5>{"Ingrédients"}</h5>
                                <div class="row">
                                    {ingredients}
                                </div>
                            </div>
                            <div class="divider"></div>
                            <div class="section">
                                <h5>{"Instructions"}</h5>
                                <div class="row">
                                    {instructions}
                                </div>
                            </div>
                            <div class="divider"></div>
                            <div class="section">
                                <div class="row">
                                    <form class="col s12">
                                        {source_url}
                                    </form>
                                </div>
                            </div>
                        </div>
                    </>
                }
            }
            None => html! {
                <>
                    <Token/>
                </>
            },
        }
    }
}

pub type RecipePage = SharedStateComponent<RecipePageComponent>;

use wasm_bindgen::prelude::*;
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
