use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

#[derive(Serialize, Deserialize)]
pub enum StandingType {
    Agent,
    #[serde(rename = "npc_corp")]
    NpcCorp,
    Faction
}

#[derive(Serialize, Deserialize)]
pub enum VisStatus {
    Public,
    Private,
}
