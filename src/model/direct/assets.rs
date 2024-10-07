use crate::model::{LocationFlag, LocationType};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Asset {
    pub is_blueprint_copy: Option<bool>,
    pub is_singleton: bool,
    pub item_id: i64,
    pub location_flag: LocationFlag,
    pub location_id: i64,
    pub location_type: LocationType,
    pub quantity: i32,
    pub type_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct AssetName {
    pub item_id: i64,
    pub name: String
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub item_id: i64,
    pub type_id: i32,
}