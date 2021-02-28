use actix_web::http::Method;
use maplit::hashmap;
use maplit::hashset;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::collections::HashSet;

pub static SECURITY_MATRIX: Lazy<HashMap<(&str, Method), HashSet<&str>>> = Lazy::new(|| {
    hashmap! {
        ("/access_token", Method::POST) => hashset![
        ],
        ("/info", Method::GET) => hashset![
        ],
        ("/meal_plans", Method::GET) => hashset![
        ],
        ("/meal_plans", Method::PUT) => hashset![
        ],
        ("/recipes", Method::GET) => hashset![
        ],
        ("/recipes", Method::POST) => hashset![
        ],
        ("/recipes/{recipe_id}", Method::GET) => hashset![
        ],
        ("/recipes/{recipe_id}", Method::PUT) => hashset![
        ],
        ("/recipes/{recipe_id}", Method::DELETE) => hashset![
        ],
        ("/shopping_list", Method::GET) => hashset![
        ],
    }
});
