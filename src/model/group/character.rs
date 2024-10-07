use crate::model::{CorpRoles, DateTime, Graphic, LocationType, StandingType, VisStatus};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Character {
    pub alliance_id: Option<i32>,
    pub birthday: DateTime,
    pub bloodline_id: i32,
    pub corporation_id: i32,
    pub description: Option<String>,
    pub faction_id: Option<i32>,
    pub gender: Gender,
    pub name: String,
    pub race_id: i32,
    pub security_status: Option<f32>,
    pub title: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Serialize, Deserialize)]
pub struct AgentsResearch {
    pub agent_id: i32,
    pub points_per_day: f32,
    pub remainder_points: f32,
    pub skill_type_id: i32,
    pub started_at: DateTime,
}

#[derive(Serialize, Deserialize)]
pub struct CorpHistory {
    pub corporation_id: i32,
    pub is_deleted: Option<bool>,
    pub record_id: i32,
    pub start_date: DateTime,
}

#[derive(Serialize, Deserialize)]
pub struct JumpFatigue {
    pub jump_fatigue_expire_date: Option<DateTime>,
    pub last_jump_date: Option<DateTime>,
    pub last_update_date: Option<DateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct Medal {
    pub corportion_id: i32,
    pub date: DateTime,
    pub description: String,
    pub graphic: Vec<Graphic>,
    pub issuer_id: i32,
    pub medal_id: i32,
    pub reason: i32,
    pub status: VisStatus,
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct Portrait {
    pub px128x128: Option<String>,
    pub px256x256: Option<String>,
    pub px512x512: Option<String>,
    pub px64x64: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Roles{
    pub roles: Vec<CorpRoles>,
    pub roles_at_base: Vec<CorpRoles>,
    pub roles_at_hq: Vec<CorpRoles>,
    pub roles_at_other: Vec<CorpRoles>
}

#[derive(Serialize, Deserialize)]
pub struct Standing {
    pub from_id: i32,
    pub from_type: StandingType,
    pub standing: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Title {
    pub name: Option<String>,
    pub title_id: Option<i32>
}

#[derive(Serialize, Deserialize)]
pub struct Affiliation {
    alliance_id: Option<i32>,
    character_id: i32,
    corporation_id: i32,
    faction_id: Option<i32>
}

#[derive(Serialize, Deserialize)]
pub struct CloneInfo {
    pub home_location: Option<HomeStation>,
    pub jump_clones: Vec<JumpClone>,
    pub lase_clone_jump_date: Option<DateTime>,
    pub last_station_change_date: Option<DateTime>
}

#[derive(Serialize, Deserialize)]
pub struct HomeStation {
    pub location_id: Option<i64>,
    pub location_type: Option<CloneLocationType>
}

#[derive(Serialize, Deserialize)]
pub enum CloneLocationType {
    Station,
    Structure
}

#[derive(Serialize, Deserialize)]
pub struct JumpClone {
    pub implants: Vec<i32>,
    pub jump_clone_id: i32,
    pub location_id: i64,
    pub location_type: LocationType,
    pub name: Option<String>
}