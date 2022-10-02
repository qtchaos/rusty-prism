mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;
use api::product_api::get_cart;
use api::product_api::get_product;
use api::product_api::get_product_fuzzy;
use api::product_api::get_random;
use repository::mongodb_repo::MongoRepo;
use std::env;

#[launch]
fn rocket() -> _ {
    env::set_var("RUST_BACKTRACE", "1");
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        // .mount("/", routes![get_cart])
        .mount("/", routes![get_cart])
        .mount("/", routes![get_product])
        .mount("/", routes![get_product_fuzzy])
        .mount("/", routes![get_random])
}
