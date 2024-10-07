use crate::model::LocationFlag;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Blueprint {
    pub item_id: i32,
    pub location_flag: LocationFlag,
    pub location_id: i64,
    pub material_efficiency: i32,
    pub quantity: i32,
    pub runs: i32,
    pub time_efficiency: i32,
    pub type_id: i32,
}