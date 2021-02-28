use crate::components::Tabs;
use crate::{
    components::Token,
    root::{AppRoute, DataHandle},
    services::{Error, MealPlansService},
};
use oikos_api::components::schemas::ShoppingList;
use yew::prelude::*;
use yew_router::{
    agent::RouteRequest,
    prelude::{Route, RouteAgentDispatcher},
    RouterState,
};
use yew_state::SharedStateComponent;
use yewtil::NeqAssign;

pub struct ShoppingListPageComponent<STATE: RouterState = ()> {
    handle: DataHandle,
    meal_plans_service: MealPlansService,
    router: RouteAgentDispatcher<STATE>,
    link: ComponentLink<Self>,
}

pub enum Message {
    ChangeRoute(AppRoute),
    ShoppingListResponse(Result<ShoppingList, Error>),
}

impl<STATE: RouterState> ShoppingListPageComponent<STATE> {
    fn get_shopping_list(&mut self) {
        self.meal_plans_service
            .get_shopping_list(self.link.callback(Message::ShoppingListResponse));
    }

    fn get_ingredients(&self) -> Html {
        if let Some(shopping_list) = &self.handle.state().shopping_list {
            shopping_list
                .items
                .iter()
                .map(|shopping_category| {
                    let ingredients = shopping_category
                        .items
                        .iter()
                        .map(|shopping_item| {
                            html! {
                                <div class="ingredient-cell">
                                    <div class="ingredient-item z-depth-1 vegetable">
                                        <img class="fit-picture"
                                            src={shopping_item.icon.clone()}/>
                                        <p>{shopping_item.ingredient.clone()}</p>
                                    </div>
                                </div>
                            }
                        })
                        .collect::<Html>();
                    html! {
                        <>
                            <h5>{shopping_category.name.clone()}</h5>
                            <div class="ingredients">
                                {ingredients}
                            </div>
                        </>
                    }
                })
                .collect::<Html>()
        } else {
            html! {}
        }
    }
}

impl<STATE: RouterState> Component for ShoppingListPageComponent<STATE> {
    type Message = Message;
    type Properties = DataHandle;
    fn create(handle: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            meal_plans_service: MealPlansService::new(),
            router: RouteAgentDispatcher::new(),
            handle,
            link,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.get_shopping_list();
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::ShoppingListResponse(shopping_list) => {
                let shopping_list = self
                    .meal_plans_service
                    .get_shopping_list
                    .response(shopping_list);
                self.handle
                    .reduce(move |state| state.shopping_list = shopping_list);
            }
            Message::ChangeRoute(route) => {
                let route = Route::from(route);
                self.router.send(RouteRequest::ChangeRoute(route));
            }
        }
        true
    }

    fn change(&mut self, handle: Self::Properties) -> ShouldRender {
        self.handle.neq_assign(handle)
    }

    fn view(&self) -> Html {
        let ingredients = self.get_ingredients();
        html! {
            <>
                <Token/>
                <Tabs title="Listes"/>
                {ingredients}
            </>
        }
    }
}

pub type ShoppingListPage = SharedStateComponent<ShoppingListPageComponent>;
