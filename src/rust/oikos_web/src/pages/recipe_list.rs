use crate::components::Tabs;
use crate::components::Token;
use crate::root::{AppRoute, DataHandle};
use crate::{
    date::next_seven_days,
    services::{Error, RecipeService},
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
    link: ComponentLink<Self>,
    router: RouteAgentDispatcher<STATE>,
    task: Option<FetchTask>,
    response: Callback<Result<RecipeList, Error>>,
}

pub enum Message {
    Response(Result<RecipeList, Error>),
    ChangeRoute(AppRoute),
    OpenModal,
}

impl<STATE: RouterState> RecipeListPageComponent<STATE> {
    fn request(&mut self) {
        self.task = Some(self.recipes_service.get_recipes(self.response.clone()));
    }
}

impl<STATE: RouterState> Component for RecipeListPageComponent<STATE> {
    type Message = Message;
    type Properties = DataHandle;
    fn create(handle: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            handle,
            recipes_service: RecipeService::new(),
            response: link.callback(Message::Response),
            link,
            router: RouteAgentDispatcher::new(),
            task: None,
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
            Message::OpenModal => unsafe { open_modal() },
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
                        let on_planning_callback = self
                            .link
                            .callback(move |_| Message::OpenModal);

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
            .map(|(_day_code, day_string)| {
                html! {
                    <li class="waves-effect">
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
