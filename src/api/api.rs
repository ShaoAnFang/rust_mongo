use crate::{models::location::Location, repository::mongodb_repo::MongoRepo};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path, Query, ServiceConfig},
    HttpResponse,
};
use mongodb::bson::oid::ObjectId;
use serde::Deserialize;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(index)
        .service(get_location)
        .service(get_locations)
        .service(get_locations2)
        .service(get_random_locations)
        .service(post_location);
}

#[derive(Debug, Deserialize)]
pub struct Request {
    count: String,
}

#[get("/")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().json("Hello world")
}

#[get("/location")]
pub async fn get_location(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let region = path.into_inner();
    if region.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let loaction = db.get_location(&region).await;

    match loaction {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
//locations/桃園市/5
#[get("/locations/{region}/{count}")]
pub async fn get_locations(db: Data<MongoRepo>, path: Path<(String, String)>) -> HttpResponse {
    // let region = path.into_inner();
    let (region, count) = path.into_inner();
    if region.is_empty() {
        return HttpResponse::BadRequest().body("invalid region");
    }
    let loaction = db.get_locations(region, count).await;

    match loaction {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

//locations2/桃園市?count=5
#[get("/locations2/{region}")]
pub async fn get_locations2(
    db: Data<MongoRepo>,
    path: Path<(String)>,
    info: Query<(Request)>,
) -> HttpResponse {
    let region = path.into_inner();
    let count = info.count.clone();

    let loaction = db.get_locations(region, count).await;

    match loaction {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

//random/locations/桃園市/5
#[get("/random/locations/{region}/{count}")]
pub async fn get_random_locations(
    db: Data<MongoRepo>,
    path: Path<(String, String)>,
) -> HttpResponse {
    // let region = path.into_inner();
    let (region, count) = path.into_inner();
    if region.is_empty() {
        return HttpResponse::BadRequest().body("invalid region");
    }
    let loaction = db.get_random_locations(region, count.clone()).await;

    match loaction {
        Ok(user) => {
            println!("get_random_locations count {}", count);
            return HttpResponse::Ok().json(user);
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/location")]
pub async fn post_location(db: Data<MongoRepo>, new_location: Json<Location>) -> HttpResponse {
    let data = Location {
        name: new_location.name.to_owned(),
        add: new_location.add.to_owned(),
        description: new_location.description.to_owned(),
        region: new_location.region.to_owned(),
    };
    let location_detail = db.post_location(data).await;
    match location_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
