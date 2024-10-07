use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Graphic {
    pub color: Option<i32>,
    pub graphic: String,
    pub layer: i32,
    pub part: i32
}