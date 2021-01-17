#![allow(clippy::unit_arg, clippy::redundant_clone)]

#[cfg(feature = "mock-server")]
pub mod mock;

use crate::models::*;
use actix_multipart::Multipart;
use actix_web::{dev::HttpResponseBuilder, http::StatusCode, web::*, HttpResponse, Responder};
use async_trait::async_trait;
use futures::{StreamExt, TryStreamExt};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;

#[async_trait(?Send)]
pub trait OikosApi {
    type Error: std::error::Error;

    async fn get_oauth_access_token(
        &self,
        _parameters: get_oauth_access_token::Parameters,
        _body: get_oauth_access_token::Body,
    ) -> Result<get_oauth_access_token::Success, get_oauth_access_token::Error<Self::Error>> {
        unimplemented!()
    }

    async fn get_info(
        &self,
        _parameters: get_info::Parameters,
    ) -> Result<get_info::Success, get_info::Error<Self::Error>> {
        unimplemented!()
    }

    async fn get_recipes(
        &self,
        _parameters: get_recipes::Parameters,
    ) -> Result<get_recipes::Success, get_recipes::Error<Self::Error>> {
        unimplemented!()
    }

    async fn add_recipe(
        &self,
        _parameters: add_recipe::Parameters,
        _body: add_recipe::Body,
    ) -> Result<add_recipe::Success, add_recipe::Error<Self::Error>> {
        unimplemented!()
    }

    async fn get_recipe_by_id(
        &self,
        _parameters: get_recipe_by_id::Parameters,
    ) -> Result<get_recipe_by_id::Success, get_recipe_by_id::Error<Self::Error>> {
        unimplemented!()
    }

    async fn update_recipe_by_id(
        &self,
        _parameters: update_recipe_by_id::Parameters,
        _body: update_recipe_by_id::Body,
    ) -> Result<update_recipe_by_id::Success, update_recipe_by_id::Error<Self::Error>> {
        unimplemented!()
    }

    async fn delete_recipe_by_id(
        &self,
        _parameters: delete_recipe_by_id::Parameters,
        _body: delete_recipe_by_id::Body,
    ) -> Result<delete_recipe_by_id::Success, delete_recipe_by_id::Error<Self::Error>> {
        unimplemented!()
    }
}

fn err_to_string(err: &dyn std::error::Error) -> String {
    let mut errors_str = Vec::new();
    let mut current_err = err.source();
    while let Some(err) = current_err {
        errors_str.push(err.to_string());
        current_err = err.source();
    }
    format!(
        "error: {}\n\ncaused by:\n\t{}",
        err,
        errors_str.as_slice().join("\n\t")
    )
}

/// Get oauth access token
async fn get_oauth_access_token<Server: OikosApi>(
    server: Data<Server>,
    body: Json<get_oauth_access_token::Body>,
) -> impl Responder {
    use get_oauth_access_token::*;
    let parameters = Parameters::new();
    let body = body.into_inner();

    match server.get_oauth_access_token(parameters, body).await {
        Ok(Success::Status200(response)) => {
            HttpResponseBuilder::new(StatusCode::from_u16(200).unwrap()).json(response)
        }
        Err(Error::Unknown(err)) => HttpResponse::InternalServerError().body(err_to_string(&err)),
    }
}
/// Get info
async fn get_info<Server: OikosApi>(server: Data<Server>) -> impl Responder {
    use get_info::*;
    let parameters = Parameters::new();

    match server.get_info(parameters).await {
        Ok(Success::Status200(response)) => {
            HttpResponseBuilder::new(StatusCode::from_u16(200).unwrap()).json(response)
        }
        Err(Error::Unknown(err)) => HttpResponse::InternalServerError().body(err_to_string(&err)),
    }
}
/// Get all recipes
async fn get_recipes<Server: OikosApi>(
    server: Data<Server>,
    header: get_recipes::Header,
) -> impl Responder {
    use get_recipes::*;
    let parameters = Parameters::new(header);

    match server.get_recipes(parameters).await {
        Ok(Success::Status200(response)) => {
            HttpResponseBuilder::new(StatusCode::from_u16(200).unwrap()).json(response)
        }
        Err(Error::Status401(response)) => {
            HttpResponseBuilder::new(StatusCode::from_u16(401).unwrap()).json(response)
        }
        Err(Error::Status403(response)) => {
            HttpResponseBuilder::new(StatusCode::from_u16(403).unwrap()).json(response)
        }
        Err(Error::Unknown(err)) => HttpResponse::InternalServerError().body(err_to_string(&err)),
    }
}
/// Add recipe
async fn add_recipe<Server: OikosApi>(
    server: Data<Server>,
    header: add_recipe::Header,
    body: Json<add_recipe::Body>,
) -> impl Responder {
    use add_recipe::*;
    let parameters = Parameters::new(header);
    let body = body.into_inner();

    match server.add_recipe(parameters, body).await {
        Ok(Success::Status200(response)) => {
            HttpResponseBuilder::new(StatusCode::from_u16(200).unwrap()).json(response)
        }
        Err(Error::Unknown(err)) => HttpResponse::InternalServerError().body(err_to_string(&err)),
    }
}
/// Get a recipe
async fn get_recipe_by_id<Server: OikosApi>(
    server: Data<Server>,
    path: Path<get_recipe_by_id::Path>,
    header: get_recipe_by_id::Header,
) -> impl Responder {
    use get_recipe_by_id::*;
    let parameters = Parameters::new(path.into_inner(), header);

    match server.get_recipe_by_id(parameters).await {
        Ok(Success::Status200(response)) => {
            HttpResponseBuilder::new(StatusCode::from_u16(200).unwrap()).json(response)
        }
        Err(Error::Status401(response)) => {
            HttpResponseBuilder::new(StatusCode::from_u16(401).unwrap()).json(response)
        }
        Err(Error::Status403(response)) => {
            HttpResponseBuilder::new(StatusCode::from_u16(403).unwrap()).json(response)
        }
        Err(Error::Status404(response)) => {
            HttpResponseBuilder::new(StatusCode::from_u16(404).unwrap()).json(response)
        }
        Err(Error::Unknown(err)) => HttpResponse::InternalServerError().body(err_to_string(&err)),
    }
}
/// Update a recipe
async fn update_recipe_by_id<Server: OikosApi>(
    server: Data<Server>,
    path: Path<update_recipe_by_id::Path>,
    header: update_recipe_by_id::Header,
    body: Json<update_recipe_by_id::Body>,
) -> impl Responder {
    use update_recipe_by_id::*;
    let parameters = Parameters::new(path.into_inner(), header);
    let body = body.into_inner();

    match server.update_recipe_by_id(parameters, body).await {
        Ok(Success::Status200(response)) => {
            HttpResponseBuilder::new(StatusCode::from_u16(200).unwrap()).json(response)
        }
        Err(Error::Status401(response)) => {
            HttpResponseBuilder::new(StatusCode::from_u16(401).unwrap()).json(response)
        }
        Err(Error::Status403(response)) => {
            HttpResponseBuilder::new(StatusCode::from_u16(403).unwrap()).json(response)
        }
        Err(Error::Status404(response)) => {
            HttpResponseBuilder::new(StatusCode::from_u16(404).unwrap()).json(response)
        }
        Err(Error::Unknown(err)) => HttpResponse::InternalServerError().body(err_to_string(&err)),
    }
}
/// Delete a recipe
async fn delete_recipe_by_id<Server: OikosApi>(
    server: Data<Server>,
    path: Path<delete_recipe_by_id::Path>,
    header: delete_recipe_by_id::Header,
) -> impl Responder {
    use delete_recipe_by_id::*;
    let parameters = Parameters::new(path.into_inner(), header);
    let body = delete_recipe_by_id::Body {};

    match server.delete_recipe_by_id(parameters, body).await {
        Ok(Success::Status200(response)) => {
            HttpResponseBuilder::new(StatusCode::from_u16(200).unwrap()).json(response)
        }
        Err(Error::Status401(response)) => {
            HttpResponseBuilder::new(StatusCode::from_u16(401).unwrap()).json(response)
        }
        Err(Error::Status403(response)) => {
            HttpResponseBuilder::new(StatusCode::from_u16(403).unwrap()).json(response)
        }
        Err(Error::Status404(response)) => {
            HttpResponseBuilder::new(StatusCode::from_u16(404).unwrap()).json(response)
        }
        Err(Error::Unknown(err)) => HttpResponse::InternalServerError().body(err_to_string(&err)),
    }
}

pub fn config<Server: OikosApi + Send + Sync + Clone + 'static>(app: &mut ServiceConfig) {
    app.service(resource("/access_token").route(post().to(get_oauth_access_token::<Server>)))
        .service(resource("/info").route(get().to(get_info::<Server>)))
        .service(
            resource("/recipes")
                .route(get().to(get_recipes::<Server>))
                .route(post().to(add_recipe::<Server>)),
        )
        .service(
            resource("/recipes/{recipe_id}")
                .route(get().to(get_recipe_by_id::<Server>))
                .route(put().to(update_recipe_by_id::<Server>))
                .route(delete().to(delete_recipe_by_id::<Server>)),
        );
}
