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
    OnSourceUrlAdd,
    OnSourceUrlChange(String),
    OnSourceUrlDelete,
    OnQuantityAdd,
    OnQuantityAmountChange(String),
    OnQuantityUnitChange(String),
    OnQuantityDelete,
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
                        ingredient.amount = Some(amount.parse::<f64>().unwrap());
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
                        quantity.amount = quantity_amount.parse::<f64>().unwrap();
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
        let on_quantity_add_callback = self.link.callback(|_| Message::OnQuantityAdd);
        let on_source_url_add_callback = self.link.callback(|_| Message::OnSourceUrlAdd);
        let on_quantity_delete_callback = self.link.callback(|_| Message::OnQuantityDelete);
        let on_source_url_delete_callback = self.link.callback(|_| Message::OnSourceUrlDelete);

        let on_quantity_amount_change_callback = self
            .link
            .callback(|e: InputData| Message::OnQuantityAmountChange(e.value));
        let on_quantity_unit_change_callback = self
            .link
            .callback(|e: InputData| Message::OnQuantityUnitChange(e.value));
        let on_source_url_change_callback = self
            .link
            .callback(|e: InputData| Message::OnSourceUrlChange(e.value));

        match self.recipe.clone() {
            Some(recipe) => {
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
                            amount
                        } else {
                            0.0
                        };

                        let ingredient_unit = if let Some(unit) = &ingredient.unit {
                            unit.clone()
                        } else {
                            "".to_string()
                        };

                        html! {
                            <div class="row">
                                <div class="input-field col s2">
                                    <input value={ingredient_amount} oninput={on_ingredient_amount_change_callback} id="quantity" type="text" class="validate"/>
                                </div>
                                <div class="input-field col s2">
                                    <input value={ingredient_unit} oninput={on_ingredient_unit_change_callback} id="unit" type="text" class="validate"/>
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

                let quantity = match recipe.quantity {
                    Some(quantity) => html! {
                        <div class="row">
                            <div class="input-field col s3">
                                <input oninput={on_quantity_amount_change_callback} value={quantity.amount} type="text" class="validate"/>
                            </div>
                            <div class="input-field col s7">
                                <input oninput={on_quantity_unit_change_callback} value={quantity.unit} type="text" class="validate"/>
                            </div>
                            <div class="input-field col s2">
                                <a onclick={on_quantity_delete_callback} class="waves-effect waves-teal btn-flat"><i class="material-icons">{"delete"}</i></a>
                            </div>
                        </div>
                    },
                    None => html! {
                        <div class="row">
                            <div class="input-field col s12">
                                <a onclick={on_quantity_add_callback} class="waves-effect waves-teal btn-flat"><i class="material-icons">{"add"}</i>{"quantité"}</a>
                            </div>
                        </div>
                    },
                };

                let source_url = match recipe.source_url {
                    Some(source_url) => html! {
                        <div class="row">
                            <div class="input-field col s10">
                                <input oninput={on_source_url_change_callback} value={source_url} type="text" class="validate"/>
                            </div>
                            <div class="input-field col s2">
                                <a onclick={on_source_url_delete_callback} class="waves-effect waves-teal btn-flat"><i class="material-icons">{"delete"}</i></a>
                            </div>
                        </div>
                    },
                    None => html! {
                        <div class="row">
                            <div class="input-field col s12">
                                <a onclick={on_source_url_add_callback} class="waves-effect waves-teal btn-flat"><i class="material-icons">{"add"}</i>{"source"}</a>
                            </div>
                        </div>
                    },
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
                                        {quantity}
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
                                                <a onclick={on_ingredient_add_callback} class="waves-effect waves-teal btn-flat"><i class="material-icons">{"add"}</i>{"ingrédient"}</a>
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
                                                <a onclick={on_step_add_callback} class="waves-effect waves-teal btn-flat"><i class="material-icons">{"add"}</i>{"instruction"}</a>
                                            </div>
                                        </div>
                                    </form>
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
