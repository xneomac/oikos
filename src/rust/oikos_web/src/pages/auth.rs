use crate::services::set_token;
use yew::prelude::*;

pub struct AuthPage;

impl Component for AuthPage {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        set_token(None);
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div class="navbar-fixed">
                    <nav>
                        <div class="nav-wrapper">
                            <a href="#!" class="brand-logo">{"Oikos kitchen"}</a>
                        </div>
                    </nav>
                </div>
                <div>
                    <div class="section">
                        <div class="row">
                            <form class="col s12">
                                <div class="row">
                                    <div class="input-field col s12">
                                        <a href="https://github.com/login/oauth/authorize?client_id=6243e7d6a656115a9871&scope=repo,write:org" class="waves-effect waves-light btn">{"Log in with github"}</a>
                                    </div>
                                </div>
                            </form>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}
