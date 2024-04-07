use serde::{Deserialize, Serialize};

pub struct Vehicles {
    userName: String,
    vehicles: Vec<per_Vehicles>,
}

pub struct per_Vehicles {
    vehicleName: String,
    kills: u32,
    killsPerMinute: f64,
    timeIn: u32,
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
