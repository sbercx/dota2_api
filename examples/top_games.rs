extern crate dota2_api;

use dota2_api::Dota2Api;

fn main() {
    let mut dota = Dota2Api::new("your key here");
    let data = dota.get_top_live_games().expect("Couldn't get match history");
    for m in data {
        println!("Lobby ID: {}\tScores: Radiant {}:{} Dire", m.lobby_id, m.radiant_score, m.dire_score);
    }
}