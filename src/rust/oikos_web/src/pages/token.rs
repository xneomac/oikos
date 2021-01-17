use crate::{root::AppRoute, services::set_token};
use yew::prelude::*;
use yew_router::{
    agent::RouteRequest,
    prelude::{Route, RouteAgentDispatcher},
    RouterState,
};

pub struct TokenPage<STATE: RouterState = ()> {
    router: RouteAgentDispatcher<STATE>,
}

pub enum Message {
    ChangeRoute(AppRoute),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub token: String,
}

impl<STATE: RouterState> Component for TokenPage<STATE> {
    type Message = Message;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        set_token(Some(props.token));
        link.send_message(Message::ChangeRoute(AppRoute::RecipeList));
        Self {
            router: RouteAgentDispatcher::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
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
        html! {
            <></>
        }
    }
}
