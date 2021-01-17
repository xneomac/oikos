use crate::components::Tabs;
use crate::components::Token;
use yew::prelude::*;

pub struct ShoppingListPage;

impl Component for ShoppingListPage {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
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
                <Token/>
                <Tabs title="Listes"/>
                <h3 class="subheader">{"Épicerie"}</h3>
                <ul class="list">
                    <li class="waves-effect">
                        <div class="valign-wrapper">
                            <div class="title">
                                { {"Farine"} }
                            </div>
                        </div>
                    </li>
                </ul>
                <h3 class="subheader">{"Viandes"}</h3>
                <ul class="list">
                    <li class="waves-effect">
                        <div class="valign-wrapper">
                            <div class="title">
                                { {"Cotelette d'agneau"} }
                            </div>
                        </div>
                    </li>
                </ul>
                <h3 class="subheader">{"Poisson"}</h3>
                <ul class="list">
                    <li class="waves-effect">
                        <div class="valign-wrapper">
                            <div class="title">
                                { {"Lotte"} }
                            </div>
                        </div>
                    </li>
                </ul>
                <h3 class="subheader">{"Primeur"}</h3>
                <ul class="list">
                    <li class="waves-effect">
                        <div class="valign-wrapper">
                            <div class="title">
                                { {"Carotte"} }
                            </div>
                        </div>
                    </li>
                </ul>
            </>
        }
    }
}
