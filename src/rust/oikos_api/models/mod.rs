#![allow(clippy::clone_on_copy)]

pub mod components {
    pub mod schemas {
        use super::super::components;
        use serde::{Deserialize, Serialize};
        use std::collections::HashMap;
        use std::convert::TryFrom;
            /// Forbidden
pub type Forbidden = String;
        

            #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
            pub struct Info {
                #[serde(rename = "version")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub version: Option<String>,
            }

            /// Not Found
pub type Notfound = String;
        

            #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
            pub struct RecipeIngredientModel {
                #[serde(rename = "amounts")]
                pub amounts: Vec<components::schemas::RecipeIngredientModelAmounts>,
                #[serde(rename = "name")]
                pub name: String,
                #[serde(rename = "notes")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub notes: Option<Vec<String>>,
                #[serde(rename = "processing")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub processing: Option<Vec<String>>,
                #[serde(rename = "substitutions")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub substitutions: Option<Vec<components::schemas::RecipeIngredientModel>>,
                #[serde(rename = "usda_num")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub usda_num: Option<String>,
            }

            #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
            pub struct RecipeIngredientModelAmounts {
                #[serde(rename = "amount")]
                pub amount: i64,
                #[serde(rename = "unit")]
                pub unit: String,
            }

            #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
            pub struct RecipeListItem {
                #[serde(rename = "id")]
                pub id: String,
                #[serde(rename = "name")]
                pub name: String,
            }


    pub type RecipeList = Vec<RecipeListItem>;
        

            
            #[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
            pub enum RecipeModelOvenFan {
                #[serde(rename = "Off")]
                Off,
                #[serde(rename = "Low")]
                Low,
                #[serde(rename = "High")]
                High,
            }

            impl std::fmt::Display for RecipeModelOvenFan {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}",
                        match self {
                            RecipeModelOvenFan::Off => "Off",
                            RecipeModelOvenFan::Low => "Low",
                            RecipeModelOvenFan::High => "High",
                        }
                    )
                }
            }
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
            pub struct RecipeModel {
                #[serde(rename = "id")]
                pub id: String,
                #[serde(rename = "ingredients")]
                pub ingredients: Vec<components::schemas::RecipeIngredientModel>,
                #[serde(rename = "name")]
                pub name: String,
                #[serde(rename = "notes")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub notes: Option<Vec<String>>,
                #[serde(rename = "oven_fan")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub oven_fan: Option<RecipeModelOvenFan>,
                #[serde(rename = "oven_temp")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub oven_temp: Option<components::schemas::RecipeModelOvenTemp>,
                #[serde(rename = "source_authors")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub source_authors: Option<Vec<String>>,
                #[serde(rename = "source_book")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub source_book: Option<components::schemas::RecipeModelSourceBook>,
                #[serde(rename = "source_url")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub source_url: Option<String>,
                #[serde(rename = "steps")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub steps: Option<Vec<components::schemas::RecipeModelSteps>>,
                #[serde(rename = "yields")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub yields: Option<Vec<components::schemas::RecipeModelYields>>,
            }

            #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
            pub struct RecipeModelHaccp {
                #[serde(rename = "control_point")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub control_point: Option<String>,
                #[serde(rename = "critical_control_point")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub critical_control_point: Option<String>,
            }

            #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
            pub struct RecipeModelOvenTemp {
                #[serde(rename = "amount")]
                pub amount: i64,
                #[serde(rename = "unit")]
                pub unit: components::schemas::TempUnit,
            }

            #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
            pub struct RecipeModelSourceBook {
                #[serde(rename = "authors")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub authors: Option<Vec<String>>,
                #[serde(rename = "isbn")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub isbn: Option<String>,
                #[serde(rename = "notes")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub notes: Option<Vec<String>>,
                #[serde(rename = "title")]
                pub title: String,
            }

            #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
            pub struct RecipeModelSteps {
                #[serde(rename = "haccp")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub haccp: Option<components::schemas::RecipeModelHaccp>,
                #[serde(rename = "notes")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub notes: Option<Vec<String>>,
                #[serde(rename = "step")]
                pub step: String,
            }

            #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
            pub struct RecipeModelYields {
                #[serde(rename = "amount")]
                pub amount: i64,
                #[serde(rename = "unit")]
                pub unit: String,
            }

            
            #[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
            pub enum TempUnit {
                #[serde(rename = "C")]
                C,
                #[serde(rename = "F")]
                F,
            }

            impl std::fmt::Display for TempUnit {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}",
                        match self {
                            TempUnit::C => "C",
                            TempUnit::F => "F",
                        }
                    )
                }
            }

            /// Unauthorized
pub type Unauthorized = String;
        

    }
}

pub mod get_info {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_info operation
    pub struct Parameters;

    impl Parameters {
        #[allow(clippy::new_without_default)]
        pub fn new(
        ) -> Self {
            Self {
            }
        }

        

        

        
    }

    

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Unspecified(T),
    }

    /// OK
pub type Response200 = components::schemas::Info;


    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    /// OK
pub type Status200 = components::schemas::Info;

}









pub mod get_recipes {
    use super::components;
    use serde::{Deserialize, Serialize};#[cfg(feature = "server")]
    use actix_web::error::ErrorBadRequest;
    #[cfg(feature = "server")]
    use actix_web::{dev, FromRequest, HttpRequest};
    #[cfg(feature = "server")]
    use futures::future::{err, ok, Ready};

    /// Parameters for get_recipes operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        
        pub authorization: String,
    }

    impl Parameters {
        pub fn new(header: Header,
        ) -> Self {
            Self {
                authorization: header.authorization,
            }
        }

        

        

        pub fn header(&self) -> Header {
            Header {authorization: self.authorization.clone(),
            }
        }
    }/// Header parameters for get_recipes operation
#[derive(Deserialize, Serialize)]
pub struct Header {
    
    pub authorization: String,
}

    #[cfg(feature = "server")]
    impl FromRequest for Header {
        type Error = actix_web::Error;
        type Future = Ready<Result<Self, Self::Error>>;
        type Config = ();

        fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
            let headers = req.headers();
            ok(Self {authorization: match headers.get("authorization") {
                Some(value) => match value.to_str() {
                    Ok(value) => value.to_string(),
                    Err(_) => return err(ErrorBadRequest("authorization should be a string")),
                },
                None => return err(ErrorBadRequest("missing authorization in header")),
            },
            })
        }
    }

    

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response401(Response401),
        Response403(Response403),
        Unspecified(T),
    }

    /// OK
pub type Response200 = components::schemas::RecipeList;
/// Unauthorized
pub type Response401 = components::schemas::Unauthorized;
/// Forbidden
pub type Response403 = components::schemas::Forbidden;


    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status401(Status401),
        Status403(Status403),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                    Self::Status401(status) => write!(f, "status 401: {:?}", status),
                    Self::Status403(status) => write!(f, "status 403: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    /// OK
pub type Status200 = components::schemas::RecipeList;
/// Unauthorized
pub type Status401 = components::schemas::Unauthorized;
/// Forbidden
pub type Status403 = components::schemas::Forbidden;

}



pub mod add_recipe {
    use super::components;
    use serde::{Deserialize, Serialize};#[cfg(feature = "server")]
    use actix_web::error::ErrorBadRequest;
    #[cfg(feature = "server")]
    use actix_web::{dev, FromRequest, HttpRequest};
    #[cfg(feature = "server")]
    use futures::future::{err, ok, Ready};

    /// Parameters for add_recipe operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        
        pub authorization: String,
    }

    impl Parameters {
        pub fn new(header: Header,
        ) -> Self {
            Self {
                authorization: header.authorization,
            }
        }

        

        

        pub fn header(&self) -> Header {
            Header {authorization: self.authorization.clone(),
            }
        }
    }/// Header parameters for add_recipe operation
#[derive(Deserialize, Serialize)]
pub struct Header {
    
    pub authorization: String,
}

    #[cfg(feature = "server")]
    impl FromRequest for Header {
        type Error = actix_web::Error;
        type Future = Ready<Result<Self, Self::Error>>;
        type Config = ();

        fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
            let headers = req.headers();
            ok(Self {authorization: match headers.get("authorization") {
                Some(value) => match value.to_str() {
                    Ok(value) => value.to_string(),
                    Err(_) => return err(ErrorBadRequest("authorization should be a string")),
                },
                None => return err(ErrorBadRequest("missing authorization in header")),
            },
            })
        }
    }

    pub type Body = components::schemas::RecipeModel;


    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Unspecified(T),
    }

    /// OK
pub type Response200 = components::schemas::RecipeModel;


    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    /// OK
pub type Status200 = components::schemas::RecipeModel;

}







pub mod get_recipe_by_id {
    use super::components;
    use serde::{Deserialize, Serialize};#[cfg(feature = "server")]
    use actix_web::error::ErrorBadRequest;
    #[cfg(feature = "server")]
    use actix_web::{dev, FromRequest, HttpRequest};
    #[cfg(feature = "server")]
    use futures::future::{err, ok, Ready};

    /// Parameters for get_recipe_by_id operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        /// id of a recipe
        pub recipe_id: String,
        
        pub authorization: String,
    }

    impl Parameters {
        pub fn new(path: Path,header: Header,
        ) -> Self {
            Self {
                recipe_id: path.recipe_id,
                authorization: header.authorization,
            }
        }

        

        pub fn path(&self) -> Path {
            Path {recipe_id: self.recipe_id.clone(),
            }
        }

        pub fn header(&self) -> Header {
            Header {authorization: self.authorization.clone(),
            }
        }
    }/// Path parameters for get_recipe_by_id operation
#[derive(Deserialize, Serialize)]
pub struct Path {
    /// id of a recipe
    pub recipe_id: String,
}/// Header parameters for get_recipe_by_id operation
#[derive(Deserialize, Serialize)]
pub struct Header {
    
    pub authorization: String,
}

    #[cfg(feature = "server")]
    impl FromRequest for Header {
        type Error = actix_web::Error;
        type Future = Ready<Result<Self, Self::Error>>;
        type Config = ();

        fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
            let headers = req.headers();
            ok(Self {authorization: match headers.get("authorization") {
                Some(value) => match value.to_str() {
                    Ok(value) => value.to_string(),
                    Err(_) => return err(ErrorBadRequest("authorization should be a string")),
                },
                None => return err(ErrorBadRequest("missing authorization in header")),
            },
            })
        }
    }

    

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response401(Response401),
        Response403(Response403),
        Response404(Response404),
        Unspecified(T),
    }

    /// OK
pub type Response200 = components::schemas::RecipeModel;
/// Unauthorized
pub type Response401 = components::schemas::Unauthorized;
/// Forbidden
pub type Response403 = components::schemas::Forbidden;
/// Not Found
pub type Response404 = components::schemas::Notfound;


    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status401(Status401),
        Status403(Status403),
        Status404(Status404),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                    Self::Status401(status) => write!(f, "status 401: {:?}", status),
                    Self::Status403(status) => write!(f, "status 403: {:?}", status),
                    Self::Status404(status) => write!(f, "status 404: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    /// OK
pub type Status200 = components::schemas::RecipeModel;
/// Unauthorized
pub type Status401 = components::schemas::Unauthorized;
/// Forbidden
pub type Status403 = components::schemas::Forbidden;
/// Not Found
pub type Status404 = components::schemas::Notfound;

}




pub mod update_recipe_by_id {
    use super::components;
    use serde::{Deserialize, Serialize};#[cfg(feature = "server")]
    use actix_web::error::ErrorBadRequest;
    #[cfg(feature = "server")]
    use actix_web::{dev, FromRequest, HttpRequest};
    #[cfg(feature = "server")]
    use futures::future::{err, ok, Ready};

    /// Parameters for update_recipe_by_id operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        /// id of a recipe
        pub recipe_id: String,
        
        pub authorization: String,
    }

    impl Parameters {
        pub fn new(path: Path,header: Header,
        ) -> Self {
            Self {
                recipe_id: path.recipe_id,
                authorization: header.authorization,
            }
        }

        

        pub fn path(&self) -> Path {
            Path {recipe_id: self.recipe_id.clone(),
            }
        }

        pub fn header(&self) -> Header {
            Header {authorization: self.authorization.clone(),
            }
        }
    }/// Path parameters for update_recipe_by_id operation
#[derive(Deserialize, Serialize)]
pub struct Path {
    /// id of a recipe
    pub recipe_id: String,
}/// Header parameters for update_recipe_by_id operation
#[derive(Deserialize, Serialize)]
pub struct Header {
    
    pub authorization: String,
}

    #[cfg(feature = "server")]
    impl FromRequest for Header {
        type Error = actix_web::Error;
        type Future = Ready<Result<Self, Self::Error>>;
        type Config = ();

        fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
            let headers = req.headers();
            ok(Self {authorization: match headers.get("authorization") {
                Some(value) => match value.to_str() {
                    Ok(value) => value.to_string(),
                    Err(_) => return err(ErrorBadRequest("authorization should be a string")),
                },
                None => return err(ErrorBadRequest("missing authorization in header")),
            },
            })
        }
    }

    pub type Body = components::schemas::RecipeModel;


    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response401(Response401),
        Response403(Response403),
        Response404(Response404),
        Unspecified(T),
    }

    /// OK
pub type Response200 = components::schemas::RecipeModel;
/// Unauthorized
pub type Response401 = components::schemas::Unauthorized;
/// Forbidden
pub type Response403 = components::schemas::Forbidden;
/// Not Found
pub type Response404 = components::schemas::Notfound;


    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status401(Status401),
        Status403(Status403),
        Status404(Status404),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                    Self::Status401(status) => write!(f, "status 401: {:?}", status),
                    Self::Status403(status) => write!(f, "status 403: {:?}", status),
                    Self::Status404(status) => write!(f, "status 404: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    /// OK
pub type Status200 = components::schemas::RecipeModel;
/// Unauthorized
pub type Status401 = components::schemas::Unauthorized;
/// Forbidden
pub type Status403 = components::schemas::Forbidden;
/// Not Found
pub type Status404 = components::schemas::Notfound;

}


pub mod delete_recipe_by_id {
    use super::components;
    use serde::{Deserialize, Serialize};#[cfg(feature = "server")]
    use actix_web::error::ErrorBadRequest;
    #[cfg(feature = "server")]
    use actix_web::{dev, FromRequest, HttpRequest};
    #[cfg(feature = "server")]
    use futures::future::{err, ok, Ready};

    /// Parameters for delete_recipe_by_id operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        /// id of a recipe
        pub recipe_id: String,
        
        pub authorization: String,
    }

    impl Parameters {
        pub fn new(path: Path,header: Header,
        ) -> Self {
            Self {
                recipe_id: path.recipe_id,
                authorization: header.authorization,
            }
        }

        

        pub fn path(&self) -> Path {
            Path {recipe_id: self.recipe_id.clone(),
            }
        }

        pub fn header(&self) -> Header {
            Header {authorization: self.authorization.clone(),
            }
        }
    }/// Path parameters for delete_recipe_by_id operation
#[derive(Deserialize, Serialize)]
pub struct Path {
    /// id of a recipe
    pub recipe_id: String,
}/// Header parameters for delete_recipe_by_id operation
#[derive(Deserialize, Serialize)]
pub struct Header {
    
    pub authorization: String,
}

    #[cfg(feature = "server")]
    impl FromRequest for Header {
        type Error = actix_web::Error;
        type Future = Ready<Result<Self, Self::Error>>;
        type Config = ();

        fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
            let headers = req.headers();
            ok(Self {authorization: match headers.get("authorization") {
                Some(value) => match value.to_str() {
                    Ok(value) => value.to_string(),
                    Err(_) => return err(ErrorBadRequest("authorization should be a string")),
                },
                None => return err(ErrorBadRequest("missing authorization in header")),
            },
            })
        }
    }

    #[derive(Deserialize)]
            pub struct Body;

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response401(Response401),
        Response403(Response403),
        Response404(Response404),
        Unspecified(T),
    }

    /// OK
pub type Response200 = ();
        
/// Unauthorized
pub type Response401 = components::schemas::Unauthorized;
/// Forbidden
pub type Response403 = components::schemas::Forbidden;
/// Not Found
pub type Response404 = components::schemas::Notfound;


    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status401(Status401),
        Status403(Status403),
        Status404(Status404),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                    Self::Status401(status) => write!(f, "status 401: {:?}", status),
                    Self::Status403(status) => write!(f, "status 403: {:?}", status),
                    Self::Status404(status) => write!(f, "status 404: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    /// OK
pub type Status200 = ();
        
/// Unauthorized
pub type Status401 = components::schemas::Unauthorized;
/// Forbidden
pub type Status403 = components::schemas::Forbidden;
/// Not Found
pub type Status404 = components::schemas::Notfound;

}




