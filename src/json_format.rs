use serde::{Deserialize, Serialize};

// TODO: serialize to snake case
// https://serde.rs/field-attrs.html

#[derive(Debug, Deserialize, Serialize)]
pub struct Vehicles {
    vehicles: Vec<PerVehicles>,
}
#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PerVehicles {
    vehicleName: String,
    r#type: String,
    kills: u32,
    killsPerMinute: f64,
    timeIn: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Weapons {
    weapons: Vec<PerWeapons>,
}
#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PerWeapons {
    weaponName: String,
    r#type: String,
    kills: u32,
    killsPerMinute: f64,
    accuracy: String,
    headshots: String,
}
#[allow(non_snake_case)]
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
    pub fn fitter_by_type(&self, vehicle_type: &str) -> Vehicles {
        Vehicles {
            vehicles: self
                .vehicles
                .iter()
                .filter(|v| v.r#type == vehicle_type)
                .cloned()
                .collect(),
        }
    }
    pub fn get_top_item(&mut self) {
        self.vehicles.truncate(10);
    }
}
impl Weapons {
    pub fn sort_by_kill(&mut self) {
        self.weapons.sort_by(|a, b| b.kills.cmp(&a.kills));
    }
    pub fn fitter_by_type(&self, weapon_type: &str) -> Weapons {
        Weapons {
            weapons: self
                .weapons
                .iter()
                .filter(|w| w.r#type == weapon_type)
                .cloned()
                .collect(),
        }
    }
    pub fn get_top_item(&mut self) {
        self.weapons.truncate(10);
    }
}
