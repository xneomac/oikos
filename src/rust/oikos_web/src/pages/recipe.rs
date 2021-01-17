use crate::components::Token;
use crate::root::{AppAnchor, AppRoute};
use crate::services::{Error, RecipeService};
use oikos_api::components::schemas::*;
use yew::{prelude::*, services::fetch::FetchTask, Properties};

pub struct RecipePage {
    link: ComponentLink<Self>,
    recipe_service: RecipeService,
    recipe: Option<RecipeModel>,
    task: Option<FetchTask>,
    response: Callback<Result<RecipeModel, Error>>,
    props: Props,
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
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub recipe_id: String,
}

impl RecipePage {
    fn get_recipe(&mut self) {
        self.task = Some(
            self.recipe_service
                .get_recipe_by_id(&self.props.recipe_id, self.response.clone()),
        );
    }
}

impl Component for RecipePage {
    type Message = Message;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            recipe_service: RecipeService::new(),
            response: link.callback(Message::OnRecipeFetch),
            recipe: None,
            task: None,
            props,
            link,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.get_recipe();
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
                        let ingredient_amount = ingredient.amounts.get_mut(0).unwrap();
                        ingredient_amount.amount = amount.parse::<i64>().unwrap();
                    }
                }
            }
            Message::OnIngredientUnitChange(index, unit) => {
                if let Some(recipe) = self.recipe.as_mut() {
                    if let Some(ingredient) = recipe.ingredients.get_mut(index) {
                        let ingredient_amount = ingredient.amounts.get_mut(0).unwrap();
                        ingredient_amount.unit = unit;
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
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let on_save = self.link.callback(|_| Message::OnSave);
        let on_name_change_callback = self
            .link
            .callback(|e: InputData| Message::OnNameChange(e.value));
        let on_ingredient_add_callback = self.link.callback(|_| Message::OnIngredientAdd);
        let on_step_add_callback = self.link.callback(|_| Message::OnStepAdd);

        match self.recipe.clone() {
            Some(recipe) => {
                let ingredients = recipe
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
                let steps = match recipe
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
                                                <input value={recipe.name} oninput={on_name_change_callback} type="text" class="validate"/>
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
            None => html! {
                <>
                    <Token/>
                </>
            },
        }
    }
}
