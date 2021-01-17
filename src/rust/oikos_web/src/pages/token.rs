use crate::{
    root::AppRoute,
    services::{set_token, AuthService, Error},
};
use oikos_api::components::schemas::AccessToken;
use yew::{prelude::*, services::fetch::FetchTask};
use yew_router::{
    agent::RouteRequest,
    prelude::{Route, RouteAgentDispatcher},
    RouterState,
};

pub struct TokenPage<STATE: RouterState = ()> {
    router: RouteAgentDispatcher<STATE>,
    link: ComponentLink<Self>,
    auth_service: AuthService,
    task: Option<FetchTask>,
    response: Callback<Result<AccessToken, Error>>,
    props: Props,
}

pub enum Message {
    ChangeRoute(AppRoute),
    TokenReceived(Result<AccessToken, Error>),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub code: String,
}

impl<STATE: RouterState> TokenPage<STATE> {
    pub fn request_token(&mut self) {
        self.task = Some(
            self.auth_service
                .request_token(self.props.code.clone(), self.response.clone()),
        );
    }
}

impl<STATE: RouterState> Component for TokenPage<STATE> {
    type Message = Message;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            router: RouteAgentDispatcher::new(),
            auth_service: AuthService::new(),
            task: None,
            response: link.callback(Message::TokenReceived),
            link,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.request_token();
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::ChangeRoute(route) => {
                let route = Route::from(route);
                self.router.send(RouteRequest::ChangeRoute(route));
            }
            Message::TokenReceived(response) => match response {
                Ok(response) => {
                    set_token(Some(response.access_token));
                    self.link
                        .send_message(Message::ChangeRoute(AppRoute::RecipeList));
                }
                Err(_) => {}
            },
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
