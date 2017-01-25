#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hyper;

pub mod models;

use std::io::Read;
use hyper::Client;
use std::fmt::Display;
use hyper::status::StatusCode;
use models::*;

#[derive(Debug)]
pub enum Error {
    Http(hyper::Error),
    Json(serde_json::Error),
    Forbidden(&'static str),
    Message(String)
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Error {
        Error::Http(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::Json(e)
    }
}

#[derive(Debug)]
pub struct Dota2Api<'a> {
    http_client: Client,
    pub key: &'a str
}

impl<'a> Dota2Api<'a> {
    pub fn new(key: &'a str) -> Dota2Api<'a> {
        Dota2Api {
            http_client: Client::new(),
            key: key
        }
    }

    pub fn get_match_history(&mut self, options: Option<&MatchHistoryOptions>) -> Result<MatchHistory, Error> {
        let mut url = String::from("http://api.steampowered.com/IDOTA2Match_570/GetMatchHistory/V001/?");
        Dota2Api::add_param(&mut url, "key", self.key);
        if let Some(opt) = options {
            Dota2Api::add_optional_param(&mut url, "hero_id", opt.hero_id);
            if let Some(skill) = opt.skill {
                Dota2Api::add_param::<i32>(&mut url, "skill", i32::from(skill));
            }
            Dota2Api::add_optional_param(&mut url, "date_min", opt.date_min);
            Dota2Api::add_optional_param(&mut url, "date_max", opt.date_max);
            Dota2Api::add_optional_param(&mut url, "account_id", opt.account_id);
            Dota2Api::add_optional_param(&mut url, "league_id", opt.league_id);
            Dota2Api::add_optional_param(&mut url, "start_at_match_id", opt.start_at_match_id);
            Dota2Api::add_optional_param(&mut url, "matches_requested", opt.matches_requested);
        }
        let response = self.get(url.as_str())?;
        let data_result: MatchHistoryResult = serde_json::from_str(response.as_str())?;
        let data = data_result.result;
        Ok(data)
    }

    pub fn get_match_details(&mut self, match_id: u64) -> Result<MatchDetails, Error> {
        let mut url = String::from("http://api.steampowered.com/IDOTA2Match_570/GetMatchDetails/V001/?");
        Dota2Api::add_param(&mut url, "key", self.key);
        Dota2Api::add_param(&mut url, "match_id", match_id);
        let response = self.get(url.as_str())?;
        let data_result: MatchDetailsResult = match serde_json::from_str(response.as_str()) {
            Err(e) => {
                match serde_json::from_str::<ApiErrorResult>(response.as_str()) {
                    Err(_) => return Err(Error::Json(e)), // return original error because we could't parse an error message from the json
                    Ok(er) => return Err(Error::Message(er.result.error)) // return the parsed error message
                };
            },
            Ok(r) => r
        };
        let data = data_result.result;
        Ok(data)
    }

    fn get(&mut self, url: &str) -> Result<String, Error> {
            let mut response = self.http_client.get(url).send()?;
            let mut tmp = String::new();
            if response.status == StatusCode::Forbidden {
                return Err(Error::Forbidden("Access is denied. Retrying will not help. Please verify your 'key' parameter."));
            }
            let _ = response.read_to_string(&mut tmp);
            Ok(tmp)
    }

    fn add_param<T: Display>(url: &mut String, param_name: &str, param_value: T) {
        url.push_str(format!("{}={}&", param_name, param_value).as_str());
    }

    fn add_optional_param<T: Display>(url: &mut String, param_name: &str, param_value: Option<T>) {
        if let Some(val) = param_value {
            url.push_str(format!("{}={}&", param_name, val).as_str());
        }
    }
}