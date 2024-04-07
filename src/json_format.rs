use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Vehicles {
    vehicles: Vec<per_Vehicles>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct per_Vehicles {
    vehicleName: String,
    kills: u32,
    killsPerMinute: f64,
    //    timeIn: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerStats {
    userName: String,
    rankName: String,
    skill: f64,
    scorePerMinute: f64,
    killsPerMinute: f64,
    winPercent: String,
    accuracy: String,
    headshots: String,
    timePlayed: String,
    killDeath: f64,
    infantryKillDeath: f64,
    infantryKillsPerMinute: f64,
    kills: u32,
    deaths: u32,
    wins: u32,
    loses: u32,
    longestHeadShot: f64,
    highestKillStreak: u32,
    roundsPlayed: u32,
}

impl Vehicles {
    pub fn sort_by_kill(&mut self) {
        self.vehicles.sort_by(|a, b| b.kills.cmp(&a.kills));
    }
    pub fn get_top_item(&mut self) {
        self.vehicles.split_off(5);
    }
}
