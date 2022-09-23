mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use api::product_api::get_product;
use api::product_api::get_product_fuzzy;
use api::product_api::get_random;
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![get_product])
        .mount("/", routes![get_product_fuzzy])
        .mount("/", routes![get_random])
}
