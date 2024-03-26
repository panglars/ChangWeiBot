use reqwest::blocking;
use serde_json::Value;
use std::error::Error;

use url::{ParseError, Url};

const STATSAPI: &str = "https://api.gametools.network/";

pub fn get_stats(name: &str) -> Result<(), Box<dyn Error>> {
    let path = "/bf1/stats";
    let base = Url::parse(STATSAPI)?;
    let mut baseurl = base.join(path)?;
    baseurl
        .query_pairs_mut()
        .append_pair("format_values", "true")
        .append_pair("name", name)
        .append_pair("platform", "pc")
        .append_pair("skip_battlelog", "false")
        .append_pair("lang", "en-us");
    let response = blocking::get(baseurl)?.json::<Value>()?;

    println!("{:#?}", response);
    Ok(())
}
