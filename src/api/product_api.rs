use crate::{models::product_model::Product, repository::mongodb_repo::MongoRepo};
use base64_stream::FromBase64Reader;
use rocket::{http::Status, serde::json::Json, State};
use std::io::Cursor;
use std::io::Read;

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

#[get("/cart?<data>&<store>&<all>")]
pub fn get_cart(
    db: &State<MongoRepo>,
    data: String,
    store: String,
    all: bool,
) -> Result<Json<Vec<Product>>, Status> {
    if store.is_empty() || data.is_empty() || data.len() > 250 {
        return Err(Status::BadRequest);
    };
    let mut reader = FromBase64Reader::new(Cursor::new(data));
    let mut data = String::new();

    reader.read_to_string(&mut data).unwrap();
    let eans: Vec<&str> = data.split(",").collect();

    let mut products: Vec<Product> = Vec::new();
    for ean in eans {
        let ean = ean.parse::<i64>().unwrap();
        let product_detail = db.get_product_ean(&ean, &store, &all);

        match product_detail {
            Ok(product) => products.push(product[0].clone()),
            Err(_) => println!("Error"),
        }
    }
    // return the products
    Ok(Json(products))
}
