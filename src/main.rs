#[macro_use]
extern crate diesel;

use deadpool_diesel::postgres::{Manager, Pool};
use std::error::Error;

mod data_types;
mod handlers;
mod models;
mod utils;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::try_init()?;

    let db_pool = init_db_pool().await?;

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(actix_web::web::Data::new(db_pool.clone()))
            .wrap(actix_web::middleware::NormalizePath::default())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_identity::IdentityService::new(
                actix_identity::CookieIdentityPolicy::new(
                    match std::env::var("SESSION_KEY") {
                        Ok(ref s) => s.as_bytes(),
                        Err(_) => &[0; 32],
                    },
                )
                .name("karyon-auth"),
            ))
            .configure(handlers::api_routes)
    })
    .bind(std::env::var("BIND_ADDRESS").unwrap_or("localhost:8080".into()))?
    .run()
    .await?;

    Ok(())
}

pub async fn init_db_pool() -> Result<Pool, Box<dyn Error>> {
    let database_url = std::env::var("DATABASE_URL")?;
    let manager = Manager::new(database_url);
    Ok(Pool::new(manager, 32))
}
