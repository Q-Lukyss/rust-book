mod game;
mod player;
mod enemy;
mod state;
mod inventory;
mod data;

use crate::player::Player;
use crate::game::run_game_loop;

fn main() {
    let hero = Player::new("Rustaciens".to_string());
    run_game_loop(hero);
}
