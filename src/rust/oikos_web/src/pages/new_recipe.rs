use crate::components::Token;
use crate::root::{AppAnchor, AppRoute};
use crate::services::{Error, RecipeService};
use oikos_api::components::schemas::RecipeModel;
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
    loading: bool,
}

pub enum Message {
    OnAdd,
    OnRecipeAdd(Result<RecipeModel, Error>),
    OnNameChange(String),
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
                quantity: None,
            },
            task: None,
            response: link.callback(Message::OnRecipeAdd),
            link,
            loading: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::OnAdd => {
                self.loading = true;
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

        let content = if self.loading {
            html! {
                <div class="container">
                    <div class="section">
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
                </div>
            }
        } else {
            html! {
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
                </div>
            }
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
                {content}
            </>
        }
    }
}
