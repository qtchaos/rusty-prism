use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub ean: i64,
    pub name: String,
    pub store: String,
    pub price: f32,
    pub unit_price: f32,
    pub is_discount: bool,
    pub is_age_restricted: bool,
    pub weight: String,
    pub url: String,
    pub image_url: String,
    pub brand: String,
}
