use crate::model::{DateTime, Position};
use crate::model::Item;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Bookmark {
    pub bookmark_id: i32,
    pub coordinates: Option<Position>,
    pub created: DateTime,
    pub creator_id: i32,
    pub folder_id: Option<i32>,
    pub item: Option<Item>,
    pub label: String,
    pub location_id: i32,
    pub notes: String
}

#[derive(Serialize, Deserialize)]
pub struct BookmarkFolder {
    pub folder_id: i32,
    pub name: String,
}