use crate::components::Tabs;
use crate::components::Token;
use crate::root::AppRoute;
use crate::services::{Error, RecipeService};
use oikos_api::components::schemas::*;
use yew::{prelude::*, services::fetch::FetchTask};
use yew_router::{
    agent::RouteRequest,
    prelude::{Route, RouteAgentDispatcher},
    RouterState,
};

pub struct RecipeListPage<STATE: RouterState = ()> {
    recipes_service: RecipeService,
    link: ComponentLink<Self>,
    router: RouteAgentDispatcher<STATE>,
    recipes: Option<RecipeList>,
    task: Option<FetchTask>,
    response: Callback<Result<RecipeList, Error>>,
}

pub enum Message {
    Response(Result<RecipeList, Error>),
    ChangeRoute(AppRoute),
}

impl<STATE: RouterState> RecipeListPage<STATE> {
    fn request(&mut self) {
        self.task = Some(self.recipes_service.get_recipes(self.response.clone()));
    }
}

impl<STATE: RouterState> Component for RecipeListPage<STATE> {
    type Message = Message;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            recipes_service: RecipeService::new(),
            response: link.callback(Message::Response),
            link,
            router: RouteAgentDispatcher::new(),
            recipes: None,
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
                self.recipes = Some(recipes);
                self.task = None;
            }
            Message::Response(Err(_)) => {
                self.task = None;
            }
            Message::ChangeRoute(route) => {
                let route = Route::from(route);
                self.router.send(RouteRequest::ChangeRoute(route));
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let on_add_callback = self
            .link
            .callback(move |_| Message::ChangeRoute(AppRoute::NewRecipe));

        let recipe_list = self
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
                <Tabs title="Recettes"/>
                <ul class="list">
                    {recipe_list}
                </ul>
                <div class="fixed-action-btn">
                    <a class="btn-floating btn-large red" onclick=on_add_callback>
                        <i class="large material-icons">{"add"}</i>
                    </a>
                </div>
            </>
        }
    }
}
