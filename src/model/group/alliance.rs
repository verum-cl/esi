use crate::model::DateTime;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Alliance {
    pub creator_corporation_id: i32,
    pub creator_id: i32,
    pub date_founded: DateTime,
    pub executor_corporation_id: Option<i32>,
    pub faction_id: Option<i32>,
    pub name: String,
    pub ticker: String
}

pub struct AllianceIcon {
    pub px128x128: Option<String>,  
    pub px64x64: Option<String>
}