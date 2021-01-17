use crate::{root::AppRoute, services::is_authenticated};
use yew::prelude::*;
use yew_router::{
    agent::RouteRequest,
    prelude::{Route, RouteAgentDispatcher},
    RouterState,
};

pub struct Token<STATE: RouterState = ()> {
    router: RouteAgentDispatcher<STATE>,
}

pub enum Message {
    ChangeRoute(AppRoute),
}

impl<STATE: RouterState> Component for Token<STATE> {
    type Message = Message;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        if !is_authenticated() {
            link.send_message(Message::ChangeRoute(AppRoute::Auth));
        }
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
