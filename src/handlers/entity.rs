use crate::{models, utils::Id, Pool};
use actix_web::{delete, get, post, web, HttpResponse};
use std::convert::{TryFrom, TryInto};

#[get("/entities/{entity_id}")]
pub async fn get_entity(
    db: web::Data<Pool>,
    token: actix_identity::Identity,
    entity_id: web::Path<Id>,
    level: web::Path<i64>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = token
        .try_into()
        .map_err(|e| actix_web::error::ErrorUnauthorized(e))?;
    let entity = models::entity::get(Pool::clone(&db), *entity_id).await?;
    Ok(HttpResponse::Ok().json(entity))
}

#[post("/entities/match/{keyword}")]
pub async fn search_entity(
    db: web::Data<Pool>,
    token: actix_identity::Identity,
    entity_id: web::Path<Id>,
    level: web::Path<i64>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = token
        .try_into()
        .map_err(|e| actix_web::error::ErrorUnauthorized(e))?;

    let entity = models::entity::get(Pool::clone(&db), *entity_id).await?;
    Ok(HttpResponse::Ok().json(entity))
}

#[post("/entities")]
pub async fn create_entity(
    db: web::Data<Pool>,
    token: actix_identity::Identity,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = token
        .try_into()
        .map_err(|e| actix_web::error::ErrorUnauthorized(e))?;

    let entity = models::entity::new(user_id);
    Ok(HttpResponse::Created()
        .json(models::entity::create(Pool::clone(&db), &entity).await?))
}

#[delete("/entities/{entity_id}")]
pub async fn remove_entity(
    db: web::Data<Pool>,
    token: actix_identity::Identity,
    entity_id: web::Path<Id>,
) -> Result<HttpResponse, actix_web::Error> {
    Id::try_from(token).map_err(|e| actix_web::error::ErrorUnauthorized(e))?;

    Ok(HttpResponse::NoContent()
        .json(models::entity::remove(Pool::clone(&db), *entity_id).await?))
}
