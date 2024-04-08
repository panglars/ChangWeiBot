use reqwest::Error;
use url::Url;

use crate::json_format::{PlayerStats, Vehicles, Weapons};

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
    Ok(json)
}

pub async fn get_vehicles(cli: reqwest::Client, name: &str) -> Result<Vehicles, Error> {
    let path = "/bf1/vehicles";
    let base = Url::parse(STATSAPI).unwrap();
    let mut url = base.join(path).unwrap();
    url.query_pairs_mut()
        .append_pair("name", name)
        .append_pair("platform", "pc")
        .append_pair("skip_battlelog", "false")
        .append_pair("lang", "en-us");
    let mut json: Vehicles = cli.get(url).send().await?.json().await?;
    json.sort_by_kill();
    json.get_top_item();
    Ok(json)
}

// BUG  error decoding response body
pub async fn get_weapons(cli: reqwest::Client, name: &str) -> Result<Weapons, Error> {
    let path = "/bf1/weapons";
    let base = Url::parse(STATSAPI).unwrap();
    let mut url = base.join(path).unwrap();
    url.query_pairs_mut()
        .append_pair("format_values", "true")
        .append_pair("name", name)
        .append_pair("platform", "pc")
        .append_pair("skip_battlelog", "false")
        .append_pair("lang", "en-us");

    // let res = cli.get(url.clone()).send().await?.text().await?;
    // println!("get api{} \n {:?}", url, res);

    let mut json: Weapons = cli.get(url.clone()).send().await?.json().await?;
    json.sort_by_kill();
    json.get_top_item();
    Ok(json)
}
