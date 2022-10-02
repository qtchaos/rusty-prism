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

#[get("/cart/<data>")]
pub fn get_cart(db: &State<MongoRepo>, data: String) -> Result<Json<Vec<Product>>, Status> {
    let data = data.replace("\"", "");

    let decoded = base64::decode(&data).unwrap();
    let decoded = String::from_utf8(decoded).unwrap();

    let s_array: Vec<&str> = decoded.split(": ").collect();
    let binding = s_array[1]
        .replace("[", "")
        .replace("]", "")
        .replace("\n", "")
        .replace("}", "");
    let eans: Vec<&str> = binding.split(",").collect();

    let mut products: Vec<Product> = Vec::new();
    for ean in eans {
        let ean = ean.parse::<i64>().unwrap();
        let product_detail = db.get_product_ean(&ean);

        match product_detail {
            Ok(product) => products.push(product),
            Err(_) => println!("Error"),
        }
    }
    // return the products
    Ok(Json(products))
}
