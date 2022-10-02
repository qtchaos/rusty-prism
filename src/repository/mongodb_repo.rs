use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::models::product_model::Product;
use core::fmt::Error;

use mongodb::bson::{self, doc};

use mongodb::{
    bson::oid::ObjectId,
    sync::{Client, Collection},
};

pub struct MongoRepo {
    col: Collection<Product>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGO_CLIENT") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable."),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database(&env::var("DB_ENV").unwrap());
        let col: Collection<Product> = db.collection(&env::var("COL2_ENV").unwrap());
        MongoRepo { col }
    }

    pub fn get_product(&self, id: &String) -> Result<Product, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let product_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting product detail.");
        Ok(product_detail.unwrap())
    }

    pub fn get_product_ean(&self, ean: &i64) -> Result<Product, Error> {
        let filter = doc! {"ean": ean};
        let product_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting product detail.");
        Ok(product_detail.unwrap())
    }

    pub fn get_product_fuzzy(&self, fuzzy: &String) -> Result<Vec<Product>, Error> {
        let pipeline = vec![
            doc! {
                "$search": {
                    "autocomplete": {
                        "query": fuzzy, "path": "name"
                    }
                },
            },
            doc! {
                "$limit": 200
            },
            doc! {
                "$project": {
                    "_id": 0
                }
            },
        ];

        let results = self
            .col
            .aggregate(pipeline, None)
            .ok()
            .expect("Error getting product detail.");

        let mut products: Vec<Product> = Vec::new();
        for result in results {
            if let Ok(item) = result {
                let product: Product = bson::from_bson(bson::Bson::Document(item)).unwrap();
                products.push(product);
            }
        }
        Ok(products)
    }

    pub fn get_random(&self) -> Result<Vec<Product>, Error> {
        let pipeline = vec![
            doc! {
                "$sample": {
                    "size": 14
                }
            },
            doc! {
                "$project": {
                    "_id": 0
                }
            },
        ];

        let results = self
            .col
            .aggregate(pipeline, None)
            .ok()
            .expect("Error getting product detail.");

        let mut products: Vec<Product> = Vec::new();
        for result in results {
            if let Ok(item) = result {
                let product: Product = bson::from_bson(bson::Bson::Document(item)).unwrap();
                products.push(product);
            }
        }
        Ok(products)
    }
}
