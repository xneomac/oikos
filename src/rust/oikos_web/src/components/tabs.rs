use crate::root::AppRoute;
use yew::prelude::*;
use yew_router::{
    agent::RouteRequest,
    prelude::{Route, RouteAgentDispatcher},
    RouterState,
};

pub struct Tabs<STATE: RouterState = ()> {
    link: ComponentLink<Self>,
    router: RouteAgentDispatcher<STATE>,
}

pub enum Message {
    ChangeRoute(AppRoute),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub title: String,
}

impl<STATE: RouterState> Component for Tabs<STATE> {
    type Message = Message;
    type Properties = Props;
    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            router: RouteAgentDispatcher::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::ChangeRoute(route) => {
                let route = Route::from(route);
                self.router.send(RouteRequest::ChangeRoute(route));
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="navbar-fixed navbar-home">
                <nav class="nav-extended">
                    <div class="nav-wrapper">
                        <a href="#!" class="brand-logo">{"Oikos kitchen"}</a>
                        <ul class="right">
                            <li>
                                <a onclick=self.link.callback(|_| Message::ChangeRoute(AppRoute::Search))>
                                    <i class="material-icons">{"search"}</i>
                                </a>
                            </li>
                        </ul>
                    </div>
                    <div class="nav-content">
                    <ul class="tabs tabs-transparent">
                        <li class="tab">
                            <a onclick=self.link.callback(|_| Message::ChangeRoute(AppRoute::RecipeList))>{"Recettes"}</a>
                        </li>
                        <li class="tab">
                            <a onclick=self.link.callback(|_| Message::ChangeRoute(AppRoute::Planning))>{"Planning"}</a>
                        </li>
                        <li class="tab">
                            <a onclick=self.link.callback(|_| Message::ChangeRoute(AppRoute::ShoppingList))>{"Liste"}</a>
                        </li>
                    </ul>
                    </div>
                </nav>
            </div>
        }
    }
}
