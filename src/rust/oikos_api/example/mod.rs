pub mod components {
    pub mod schemas {
      use crate::components;
      use serde_json::json;
      use crate::components::schemas::{
        Forbidden,
        Info,
        Notfound,
        RecipeIngredientModel,
        RecipeIngredientModelAmounts,
        RecipeList,
        RecipeModel,
        RecipeModelHaccp,
        RecipeModelOvenTemp,
        RecipeModelSourceBook,
        RecipeModelSteps,
        RecipeModelYields,
        TempUnit,
        Unauthorized,
      };
        

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

        

pub struct NotfoundExample;

    const NOTFOUND: &str = r#""not found""#;

    impl NotfoundExample {
        pub fn default() -> Option<Notfound> {
            Some(serde_json::from_str(NOTFOUND).unwrap())
        }
    }

        

pub struct RecipeIngredientModelExample;

    const RECIPE_INGREDIENT_MODEL: &str = r#"{"amounts":[{"amount":0,"unit":"unit"},{"amount":0,"unit":"unit"}],"name":"name","notes":["notes","notes"],"processing":["processing","processing"],"substitutions":[null,null],"usda_num":"usda_num"}"#;

    impl RecipeIngredientModelExample {
        pub fn default() -> Option<RecipeIngredientModel> {
            Some(serde_json::from_str(RECIPE_INGREDIENT_MODEL).unwrap())
        }
    }

        

pub struct RecipeIngredientModelAmountsExample;


        

pub struct RecipeListExample;
impl RecipeListExample {
        pub fn default() -> Option<RecipeList> {None
        }
    }

        

pub struct RecipeModelExample;

    const RECIPE_MODEL: &str = r#"{"id":"id","ingredients":[{"amounts":[{"amount":0,"unit":"unit"},{"amount":0,"unit":"unit"}],"name":"name","notes":["notes","notes"],"processing":["processing","processing"],"substitutions":[null,null],"usda_num":"usda_num"},{"amounts":[{"amount":0,"unit":"unit"},{"amount":0,"unit":"unit"}],"name":"name","notes":["notes","notes"],"processing":["processing","processing"],"substitutions":[null,null],"usda_num":"usda_num"}],"name":"name","notes":["notes","notes"],"oven_fan":"Off","oven_temp":{"amount":1},"source_authors":["source_authors","source_authors"],"source_book":{"authors":["authors","authors"],"isbn":"isbn","notes":["notes","notes"],"title":"title"},"source_url":"source_url","steps":[{"haccp":{"control_point":"control_point","critical_control_point":"critical_control_point"},"notes":["notes","notes"],"step":"step"},{"haccp":{"control_point":"control_point","critical_control_point":"critical_control_point"},"notes":["notes","notes"],"step":"step"}],"yields":[{"amount":6,"unit":"unit"},{"amount":6,"unit":"unit"}]}"#;

    impl RecipeModelExample {
        pub fn default() -> Option<RecipeModel> {
            Some(serde_json::from_str(RECIPE_MODEL).unwrap())
        }
    }

        

pub struct RecipeModelHaccpExample;


        

pub struct RecipeModelOvenTempExample;


        

pub struct RecipeModelSourceBookExample;


        

pub struct RecipeModelStepsExample;


        

pub struct RecipeModelYieldsExample;


        

pub struct TempUnitExample;
impl TempUnitExample {
        pub fn default() -> Option<TempUnit> {None
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