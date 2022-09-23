use crate::{models::product_model::Product, repository::mongodb_repo::MongoRepo};
use rocket::{http::Status, serde::json::Json, State};

#[get("/product/<path>")]
pub fn get_product(db: &State<MongoRepo>, path: String) -> Result<Json<Product>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let product_detail = db.get_product(&id);
    match product_detail {
        Ok(product) => Ok(Json(product)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/fuzzy/<fuzzy>")]
pub fn get_product_fuzzy(
    db: &State<MongoRepo>,
    fuzzy: String,
) -> Result<Json<Vec<Product>>, Status> {
    if fuzzy.is_empty() {
        return Err(Status::BadRequest);
    };
    let product_detail = db.get_product_fuzzy(&fuzzy);
    match product_detail {
        Ok(product) => Ok(Json(product)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/random")]
pub fn get_random(db: &State<MongoRepo>) -> Result<Json<Vec<Product>>, Status> {
    let product_detail = db.get_random();
    match product_detail {
        Ok(product) => Ok(Json(product)),
        Err(_) => Err(Status::InternalServerError),
    }
}
