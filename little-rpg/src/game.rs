use crate::state::GameState;
use crate::player::Player;
use crate::enemy::Enemy;

pub fn run_game_loop(mut player: Player) {
    let mut state = GameState::Menu;

    loop {
        match state {
            GameState::Menu => {
                println!("=== MENU ===");
                state = GameState::Exploration;
            },
            GameState::Exploration => {
                println!("{}, tu explores un donjon...", player.name);
                state = GameState::Combat;
            },
            GameState::Combat => {
                let mut enemy = Enemy::spawn();
                println!("Un {} apparaît ! Il s'agit de {}.", enemy.enemy_type.as_str(), enemy.name );

                while enemy.hp > 0 && player.hp.0 > 0 {
                    println!("Tu frappes !");
                    player.attack(&mut enemy);
                    if enemy.hp > 0 {
                        println!("L'ennemi riposte !");
                        player.take_dmg(enemy.attack)
                    }
                }
                if player.hp.0 <= 0 {
                    state = GameState::GameOver;
                } else {
                    state = GameState::Exploration;
                }
            },
            GameState::GameOver => {
                println!("Tu es mort. Fin du jeu.");
                break;
            }
        }
    }
}
