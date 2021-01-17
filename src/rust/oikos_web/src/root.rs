use super::pages::*;
use yew::prelude::*;
use yew_router::{components::RouterAnchor, prelude::*};

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

pub struct RootComponent;

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
    type Properties = ();
    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
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
