use crate::model::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Event {
    pub date: DateTime,
    pub duration: i32,
    pub event_id: i32,
    pub importance: i32,
    pub owner_id: i32,
    pub owner_name: String,
    pub owner_type: OwnerType,
    pub response: Response,
    pub text: String,
    pub title: String
}

#[derive(Serialize, Deserialize)]
pub enum OwnerType {
    EveServer,
    Corporation,
    Faction,
    Character,
    Alliance
}

#[derive(Serialize, Deserialize)]
pub struct Summary {
    pub event_date: Option<DateTime>,
    pub event_id: Option<i32>,
    pub event_response: Option<Response>,
    pub importance: Option<i32>,
    pub title: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum Response {
    Declined,
    NotResponded,
    Accepted,
    Tentative,
}

#[derive(Serialize, Deserialize)]
pub struct Attendee {
    pub character_id: Option<i32>,
    pub event_response: Option<Response>,
}
