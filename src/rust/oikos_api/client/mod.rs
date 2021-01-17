#![allow(clippy::ptr_arg)]
use url::{Url};
use std::sync::Arc;
use std::time::Duration;

/* Reqwest's errors are bad-mannered and recurse on their source when displayed.
 * This behavior doesn't interact well with thiserror which also recurse on error's cause
 * when displayed. To prevent this issue, this wrapper hides the error's source from thiserror.
 */
pub struct ReqwestError {
    err: reqwest::Error,
}

impl ReqwestError {
    pub fn new(err: reqwest::Error) -> Self {
        Self { err }
    }
}

impl std::error::Error for ReqwestError {}

impl From<reqwest::Error> for ReqwestError {
    fn from(err: reqwest::Error) -> Self {
        Self::new(err)
    }
}

impl std::fmt::Debug for ReqwestError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(status) = self.err.status() {
            write!(f, "{:?}: {:?}", self.err, status)
        } else {
            std::fmt::Debug::fmt(&self.err, f)
        }
    }
}

impl std::fmt::Display for ReqwestError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(status) = self.err.status() {
            write!(f, "{}: {}", self.err, status)
        } else {
            std::fmt::Display::fmt(&self.err, f)
        }
    }
}

#[derive(Clone)]
pub struct OikosApiClient {
    pub url: Url,
    pub client: reqwest::Client,
}

impl OikosApiClient {
    pub fn new(url: &str) -> Self {
        let url = Url::parse(url).expect("cannot parse url");
        Self {
            url,
            client: reqwest::Client::new(),
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.client = reqwest::Client::builder().timeout(timeout).build().expect("bad client build");
        self
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_info(
        &self,) -> Result<get_info::Success, get_info::Error> {
        use get_info::*;
        let url = self.url.join(
            "/info".trim_start_matches('/')
        ).expect("url parse error");
        let response = self.client
            .get(url)
            
            .send().await.map_err(ReqwestError::new)?;
        match response.status().as_str() {"200" => {let response_body = response.json().await.map_err(ReqwestError::new)?;
                    Ok(Success::Status200(response_body))
                    
                }
                _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_recipes(
        &self,parameters: &get_recipes::Parameters,) -> Result<get_recipes::Success, get_recipes::Error> {
        use get_recipes::*;
        let url = self.url.join(
            "/recipes".trim_start_matches('/')
        ).expect("url parse error");
        let response = self.client
            .get(url)
            
            .send().await.map_err(ReqwestError::new)?;
        match response.status().as_str() {"200" => {let response_body = response.json().await.map_err(ReqwestError::new)?;
                    Ok(Success::Status200(response_body))
                    
                }"401" => {let response_body = response.json().await.map_err(ReqwestError::new)?;
                    Err(Error::Status401(response_body))
                }"403" => {let response_body = response.json().await.map_err(ReqwestError::new)?;
                    Err(Error::Status403(response_body))
                }
                _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn add_recipe(
        &self,parameters: &add_recipe::Parameters,body: &add_recipe::Body,
        ) -> Result<add_recipe::Success, add_recipe::Error> {
        use add_recipe::*;
        let url = self.url.join(
            "/recipes".trim_start_matches('/')
        ).expect("url parse error");
        let response = self.client
            .post(url)
            .json(&body)
            .send().await.map_err(ReqwestError::new)?;
        match response.status().as_str() {"200" => {let response_body = response.json().await.map_err(ReqwestError::new)?;
                    Ok(Success::Status200(response_body))
                    
                }
                _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_recipe_by_id(
        &self,parameters: &get_recipe_by_id::Parameters,) -> Result<get_recipe_by_id::Success, get_recipe_by_id::Error> {
        use get_recipe_by_id::*;
        let url = self.url.join(
            format!("/recipes/{recipe_id}", recipe_id = parameters.recipe_id).trim_start_matches('/')
        ).expect("url parse error");
        let response = self.client
            .get(url)
            
            .send().await.map_err(ReqwestError::new)?;
        match response.status().as_str() {"200" => {let response_body = response.json().await.map_err(ReqwestError::new)?;
                    Ok(Success::Status200(response_body))
                    
                }"401" => {let response_body = response.json().await.map_err(ReqwestError::new)?;
                    Err(Error::Status401(response_body))
                }"403" => {let response_body = response.json().await.map_err(ReqwestError::new)?;
                    Err(Error::Status403(response_body))
                }"404" => {let response_body = response.json().await.map_err(ReqwestError::new)?;
                    Err(Error::Status404(response_body))
                }
                _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn update_recipe_by_id(
        &self,parameters: &update_recipe_by_id::Parameters,body: &update_recipe_by_id::Body,
        ) -> Result<update_recipe_by_id::Success, update_recipe_by_id::Error> {
        use update_recipe_by_id::*;
        let url = self.url.join(
            format!("/recipes/{recipe_id}", recipe_id = parameters.recipe_id).trim_start_matches('/')
        ).expect("url parse error");
        let response = self.client
            .put(url)
            .json(&body)
            .send().await.map_err(ReqwestError::new)?;
        match response.status().as_str() {"200" => {let response_body = response.json().await.map_err(ReqwestError::new)?;
                    Ok(Success::Status200(response_body))
                    
                }"401" => {let response_body = response.json().await.map_err(ReqwestError::new)?;
                    Err(Error::Status401(response_body))
                }"403" => {let response_body = response.json().await.map_err(ReqwestError::new)?;
                    Err(Error::Status403(response_body))
                }"404" => {let response_body = response.json().await.map_err(ReqwestError::new)?;
                    Err(Error::Status404(response_body))
                }
                _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn delete_recipe_by_id(
        &self,parameters: &delete_recipe_by_id::Parameters,) -> Result<delete_recipe_by_id::Success, delete_recipe_by_id::Error> {
        use delete_recipe_by_id::*;
        let url = self.url.join(
            format!("/recipes/{recipe_id}", recipe_id = parameters.recipe_id).trim_start_matches('/')
        ).expect("url parse error");
        let response = self.client
            .delete(url)
            
            .send().await.map_err(ReqwestError::new)?;
        match response.status().as_str() {"200" => {let response_body = ();
                    Ok(Success::Status200(response_body))
                    
                }"401" => {let response_body = response.json().await.map_err(ReqwestError::new)?;
                    Err(Error::Status401(response_body))
                }"403" => {let response_body = response.json().await.map_err(ReqwestError::new)?;
                    Err(Error::Status403(response_body))
                }"404" => {let response_body = response.json().await.map_err(ReqwestError::new)?;
                    Err(Error::Status404(response_body))
                }
                _ => Err(Error::unknown(response).await),
        }
    }
}

pub mod get_info {
    pub use crate::models::get_info::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Timeout occured during request
        Timeout(#[from] async_std::future::TimeoutError),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown{
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_recipes {
    pub use crate::models::get_recipes::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Timeout occured during request
        Timeout(#[from] async_std::future::TimeoutError),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Status 403 error: {0:?}
        Status403(Status403),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown{
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod add_recipe {
    pub use crate::models::add_recipe::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Timeout occured during request
        Timeout(#[from] async_std::future::TimeoutError),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown{
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_recipe_by_id {
    pub use crate::models::get_recipe_by_id::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Timeout occured during request
        Timeout(#[from] async_std::future::TimeoutError),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Status 403 error: {0:?}
        Status403(Status403),
        /// Status 404 error: {0:?}
        Status404(Status404),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown{
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod update_recipe_by_id {
    pub use crate::models::update_recipe_by_id::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Timeout occured during request
        Timeout(#[from] async_std::future::TimeoutError),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Status 403 error: {0:?}
        Status403(Status403),
        /// Status 404 error: {0:?}
        Status404(Status404),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown{
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod delete_recipe_by_id {
    pub use crate::models::delete_recipe_by_id::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Timeout occured during request
        Timeout(#[from] async_std::future::TimeoutError),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Status 403 error: {0:?}
        Status403(Status403),
        /// Status 404 error: {0:?}
        Status404(Status404),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown{
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}