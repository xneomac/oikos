use crate::components::Token;
use crate::root::{AppRoute, DataHandle};
use crate::services::{Error, RecipeService};
use oikos_api::components::schemas::RecipeList;
use wasm_bindgen::prelude::*;
use yew::{prelude::*, services::fetch::FetchTask};
use yew_router::{
    agent::RouteRequest,
    prelude::{Route, RouteAgentDispatcher},
    RouterState,
};
use yew_state::SharedStateComponent;
use yewtil::NeqAssign;

pub struct SearchPageComponent<STATE: RouterState = ()> {
    handle: DataHandle,
    recipes_service: RecipeService,
    link: ComponentLink<Self>,
    router: RouteAgentDispatcher<STATE>,
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

impl<STATE: RouterState> SearchPageComponent<STATE> {
    fn request(&mut self) {
        self.task = Some(self.recipes_service.get_recipes(self.response.clone()));
    }
}

impl<STATE: RouterState> Component for SearchPageComponent<STATE> {
    type Message = Message;
    type Properties = DataHandle;
    fn create(handle: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            handle,
            recipes_service: RecipeService::new(),
            response: link.callback(Message::Response),
            link,
            router: RouteAgentDispatcher::new(),
            recipes_found: None,
            task: None,
            search: "".to_string(),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.request();
            self.link
                .send_message(Message::OnSearchChange("".to_string()));
            focus_search();
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::Response(Ok(recipes)) => {
                self.handle
                    .reduce(move |state| state.recipes = Some(recipes));
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
                if let Some(recipes) = self.handle.state().recipes.as_ref() {
                    self.recipes_found = Some(
                        recipes
                            .iter()
                            .filter(|recipe| {
                                recipe
                                    .name
                                    .to_lowercase()
                                    .contains(search.to_lowercase().as_str())
                            })
                            .cloned()
                            .collect::<Vec<_>>(),
                    );
                }
                self.search = search;
            }
        }
        true
    }

    fn change(&mut self, handle: Self::Properties) -> ShouldRender {
        self.handle.neq_assign(handle)
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
                            <input class="search" oninput=on_search_change_callback id="search" type="search" required=true/>
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

pub type SearchPage = SharedStateComponent<SearchPageComponent>;

#[wasm_bindgen(inline_js = "

        export function focus_search() {
            var elems = document.querySelectorAll('.search');
            elems[0].focus();
        }")]
extern "C" {
    fn focus_search();
}
