use super::pages::*;
use crate::data::MealPlans;
use oikos_api::components::schemas::RecipeList;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::{components::RouterAnchor, prelude::*};
use yew_state::{Area, SharedHandle, SharedStateComponent, Storable};

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
    #[to = "/recipes/{}"]
    Recipe(String),
    #[to = "/recipes"]
    RecipeList,
    #[to = "/new_recipe"]
    NewRecipe,
    #[to = "/shopping"]
    ShoppingList,
    #[to = "/search"]
    Search,
    #[to = "/auth"]
    Auth,
    #[to = "/token?code={}"]
    Token(String),
    #[to = "/!"]
    Planning,
}

pub type AppRouter = Router<AppRoute>;
pub type AppAnchor = RouterAnchor<AppRoute>;

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataState {
    pub recipes: Option<RecipeList>,
    pub meal_plans: Option<MealPlans>,
}

impl Storable for DataState {
    fn area() -> Area {
        Area::Local
    }
}

pub type DataHandle = SharedHandle<DataState>;

pub struct RootComponent {
    _handle: DataHandle,
}

impl RootComponent {
    fn switch(switch: AppRoute) -> Html {
        match switch {
            AppRoute::Recipe(id) => {
                html! { <RecipePage recipe_id=id /> }
            }
            AppRoute::RecipeList => {
                html! { <RecipeListPage /> }
            }
            AppRoute::NewRecipe => {
                html! { <NewRecipePage /> }
            }
            AppRoute::ShoppingList => {
                html! { <ShoppingListPage /> }
            }
            AppRoute::Planning => {
                html! { <PlanningPage /> }
            }
            AppRoute::Auth => {
                html! { <AuthPage /> }
            }
            AppRoute::Token(code) => {
                html! { <TokenPage code=code/> }
            }
            AppRoute::Search => {
                html! { <SearchPage /> }
            }
        }
    }
}

impl Component for RootComponent {
    type Message = ();
    type Properties = DataHandle;
    fn create(handle: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { _handle: handle }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            wasm_logger::init(wasm_logger::Config::default());
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <AppRouter
                render=AppRouter::render(Self::switch)
            />
        }
    }
}

pub type Root = SharedStateComponent<RootComponent>;
