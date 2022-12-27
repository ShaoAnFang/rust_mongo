#![allow(unused)]
mod api;
mod models;
mod repository;

use actix_web::{web::Data, App, HttpServer};
use api::api::{get_location, get_locations, index, post_location};
use repository::mongodb_repo::MongoRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .configure(api::api::init)
        // .service(index)
        // .service(get_location)
        // .service(get_locations)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
