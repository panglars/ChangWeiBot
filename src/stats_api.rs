use reqwest::Error;
use url::Url;

use serde::{Deserialize, Serialize};

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

const STATSAPI: &str = "https://api.gametools.network/";

pub async fn get_stats(cli: reqwest::Client, name: &str) -> Result<PlayerStats, Error> {
    let path = "/bf1/stats";
    let base = Url::parse(STATSAPI).unwrap();
    let mut url = base.join(path).unwrap();
    url.query_pairs_mut()
        .append_pair("format_values", "true")
        .append_pair("name", name)
        .append_pair("platform", "pc")
        .append_pair("skip_battlelog", "false")
        .append_pair("lang", "en-us");

    let json: PlayerStats = cli.get(url).send().await?.json().await?;
    //println!("{:?}", json);
    Ok(json)
}
