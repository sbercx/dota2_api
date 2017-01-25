extern crate dota2_api;

use dota2_api::Dota2Api;
use dota2_api::models::{SkillLevel, MatchHistoryOptions};


fn main() {
    let mut dota = Dota2Api::new("your key here");
    let options = MatchHistoryOptions {
        matches_requested: Some(10),
        skill: Some(SkillLevel::VeryHigh),
        ..Default::default()
    };
    let data = dota.get_match_history(Some(&options)).expect("Couldn't get match history");
    for m in data.matches {
        println!("Match ID: {}, Number of players: {}", m.match_id, m.players.len());
    }
}