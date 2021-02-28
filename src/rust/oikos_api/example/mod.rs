pub mod components {
    pub mod schemas {
        use crate::components;
        use crate::components::schemas::{
            AccessToken, AccessTokenRequest, Forbidden, Info, MealPlans, Notfound,
            RecipeIngredientModel, RecipeList, RecipeModel, RecipeModelHaccp, RecipeModelOvenTemp,
            RecipeModelQuantity, RecipeModelSourceBook, RecipeModelSteps, TempUnit, Unauthorized,
        };
        use serde_json::json;

        pub struct AccessTokenExample;

        const ACCESS_TOKEN: &str =
            r#"{"access_token":"access_token","scope":"scope","token_type":"token_type"}"#;

        impl AccessTokenExample {
            pub fn default() -> Option<AccessToken> {
                Some(serde_json::from_str(ACCESS_TOKEN).unwrap())
            }
        }

        pub struct AccessTokenRequestExample;
        impl AccessTokenRequestExample {
            pub fn default() -> Option<AccessTokenRequest> {
                let access_token_request: String = json!({}).to_string();
                Some(serde_json::from_str(&access_token_request).unwrap())
            }
        }

        pub struct ForbiddenExample;

        const FORBIDDEN: &str = r#""forbidden""#;

        impl ForbiddenExample {
            pub fn default() -> Option<Forbidden> {
                Some(serde_json::from_str(FORBIDDEN).unwrap())
            }
        }

        pub struct InfoExample;

        const INFO: &str = r#"{"version":"version"}"#;

        impl InfoExample {
            pub fn default() -> Option<Info> {
                Some(serde_json::from_str(INFO).unwrap())
            }
        }

        pub struct MealPlansExample;
        impl MealPlansExample {
            pub fn default() -> Option<MealPlans> {
                None
            }
        }

        pub struct NotfoundExample;

        const NOTFOUND: &str = r#""not found""#;

        impl NotfoundExample {
            pub fn default() -> Option<Notfound> {
                Some(serde_json::from_str(NOTFOUND).unwrap())
            }
        }

        pub struct RecipeIngredientModelExample;

        const RECIPE_INGREDIENT_MODEL: &str = r#"{"amount":0.8008281904610115,"category":"category","icon":"icon","name":"name","notes":["notes","notes"],"processing":["processing","processing"],"substitutions":[null,null],"unit":"unit","usda_num":"usda_num"}"#;

        impl RecipeIngredientModelExample {
            pub fn default() -> Option<RecipeIngredientModel> {
                Some(serde_json::from_str(RECIPE_INGREDIENT_MODEL).unwrap())
            }
        }

        pub struct RecipeListExample;
        impl RecipeListExample {
            pub fn default() -> Option<RecipeList> {
                None
            }
        }

        pub struct RecipeModelExample;

        const RECIPE_MODEL: &str = r#"{"id":"id","ingredients":[{"amount":0.8008281904610115,"category":"category","icon":"icon","name":"name","notes":["notes","notes"],"processing":["processing","processing"],"substitutions":[null,null],"unit":"unit","usda_num":"usda_num"},{"amount":0.8008281904610115,"category":"category","icon":"icon","name":"name","notes":["notes","notes"],"processing":["processing","processing"],"substitutions":[null,null],"unit":"unit","usda_num":"usda_num"}],"name":"name","notes":["notes","notes"],"oven_fan":"Off","oven_temp":{"amount":1},"quantity":{"amount":6.027456183070403,"unit":"unit"},"source_authors":["source_authors","source_authors"],"source_book":{"authors":["authors","authors"],"isbn":"isbn","notes":["notes","notes"],"title":"title"},"source_url":"source_url","steps":[{"haccp":{"control_point":"control_point","critical_control_point":"critical_control_point"},"notes":["notes","notes"],"step":"step"},{"haccp":{"control_point":"control_point","critical_control_point":"critical_control_point"},"notes":["notes","notes"],"step":"step"}]}"#;

        impl RecipeModelExample {
            pub fn default() -> Option<RecipeModel> {
                Some(serde_json::from_str(RECIPE_MODEL).unwrap())
            }
        }

        pub struct RecipeModelHaccpExample;

        pub struct RecipeModelOvenTempExample;

        pub struct RecipeModelQuantityExample;

        pub struct RecipeModelSourceBookExample;

        pub struct RecipeModelStepsExample;

        pub struct TempUnitExample;
        impl TempUnitExample {
            pub fn default() -> Option<TempUnit> {
                None
            }
        }

        pub struct UnauthorizedExample;

        const UNAUTHORIZED: &str = r#""unauthorized""#;

        impl UnauthorizedExample {
            pub fn default() -> Option<Unauthorized> {
                Some(serde_json::from_str(UNAUTHORIZED).unwrap())
            }
        }
    }
}
