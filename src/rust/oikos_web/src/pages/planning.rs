use crate::components::Tabs;
use crate::components::Token;
use yew::prelude::*;

pub struct PlanningPage;

impl Component for PlanningPage {
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
                <Tabs title="Planning"/>
                <div class="container">
                    <div class="row">
                        <div class="col s12 m6">
                        <div class="card">
                            <div class="card-image">
                                <img src="https://www.tipiak.fr/sites/default/files/styles/image_detail/public/recettes/images/Couscous-marocain-TIPIAK.jpg?itok=Ez-4eRy5"/>
                            </div>
                            <div class="card-content">
                            <span class="card-title">{"Couscous"}</span>
                            <p>{"Lundi 11 janvier"}</p>
                            </div>
                        </div>
                        </div>
                    </div>

                    <div class="row">
                        <div class="col s12 m6">
                        <div class="card">
                            <div class="card-image">
                                <img src="https://img.cuisineaz.com/660x660/2018-11-09/i144059-garniture-pour-fajitas-au-cookeo.jpeg"/>
                            </div>
                            <div class="card-content">
                            <span class="card-title">{"Fajitas"}</span>
                            <p>{"Mardi 12 janvier"}</p>
                            </div>
                        </div>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}
