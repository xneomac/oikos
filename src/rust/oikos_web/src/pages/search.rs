use crate::components::Token;
use crate::root::AppRoute;
use crate::services::{Error, RecipeService};
use oikos_api::components::schemas::RecipeList;
use yew::{prelude::*, services::fetch::FetchTask};
use yew_router::{
    agent::RouteRequest,
    prelude::{Route, RouteAgentDispatcher},
    RouterState,
};

pub struct SearchPage<STATE: RouterState = ()> {
    recipes_service: RecipeService,
    link: ComponentLink<Self>,
    router: RouteAgentDispatcher<STATE>,
    recipes: Option<RecipeList>,
    recipes_found: Option<RecipeList>,
    search: String,
    task: Option<FetchTask>,
    response: Callback<Result<RecipeList, Error>>,
}

pub enum Message {
    Response(Result<RecipeList, Error>),
    ChangeRoute(AppRoute),
    OnSearchChange(String),
}

impl<STATE: RouterState> SearchPage<STATE> {
    fn request(&mut self) {
        self.task = Some(self.recipes_service.get_recipes(self.response.clone()));
    }
}

impl<STATE: RouterState> Component for SearchPage<STATE> {
    type Message = Message;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            recipes_service: RecipeService::new(),
            response: link.callback(Message::Response),
            link,
            router: RouteAgentDispatcher::new(),
            recipes: None,
            recipes_found: None,
            task: None,
            search: "".to_string(),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.request();
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::Response(Ok(recipes)) => {
                self.recipes = Some(recipes.clone());
                self.recipes_found = Some(recipes);
                self.task = None;
            }
            Message::Response(Err(_)) => {
                self.task = None;
            }
            Message::ChangeRoute(route) => {
                let route = Route::from(route);
                self.router.send(RouteRequest::ChangeRoute(route));
            }
            Message::OnSearchChange(search) => {
                if let Some(recipes) = self.recipes.as_ref() {
                    self.recipes_found = Some(
                        recipes
                            .iter()
                            .filter(|recipe| recipe.name.contains(search.as_str()))
                            .cloned()
                            .collect::<Vec<_>>(),
                    );
                }
                self.search = search;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let on_search_change_callback = self
            .link
            .callback(|e: InputData| Message::OnSearchChange(e.value));
        let on_close_click = self
            .link
            .callback(move |_| Message::ChangeRoute(AppRoute::RecipeList));

        let recipe_list = self
            .recipes_found
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

                        html! {
                            <li class="waves-effect" onclick=onclick>
                                <div class="valign-wrapper">
                                    <div class="title">
                                        { recipe.name.clone() }
                                    </div>
                                    <i class="material-icons ml-auto">{"info"}</i>
                                </div>
                            </li>
                        }
                    })
                    .collect::<Html>()
            })
            .unwrap_or_else(|| html! { <></> });

        html! {
            <>
                <Token/>
                <div class="navbar-fixed">
                    <nav>
                        <div class="nav-wrapper">
                        <form>
                            <div class="input-field">
                            <input oninput=on_search_change_callback id="search" type="search" required=true/>
                            <label class="label-icon" for="search"><i class="material-icons">{"search"}</i></label>
                            <i onclick=on_close_click class="material-icons">{"close"}</i>
                            </div>
                        </form>
                        </div>
                    </nav>
                </div>
                <ul class="list">
                    {recipe_list}
                </ul>
            </>
        }
    }
}
