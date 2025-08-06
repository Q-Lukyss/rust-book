use crate::state::GameState;
use crate::player::Player;
use crate::enemy::{Enemy, EnemyRank, EnemyType};
use std::io::{self, Write};

pub fn run_game_loop(mut player: Player) {
    let mut state = GameState::Menu;

    loop {
        match state {
            GameState::Menu => {
                display_menu();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                match input.trim() {
                    "1" => state = GameState::Menu,
                    "2" => state = GameState::Combat,
                    "3" => {
                        println!("Fin du jeu !");
                        std::process::exit(0);
                    }
                    _ => {
                        println!("Choix invalide");
                        state = GameState::Menu;
                    }
                }
            },
            GameState::Exploration => {
                println!("{}, tu explores un donjon...", player.name);
                state = GameState::Combat;
            },
            GameState::Combat => {
                // choisir le type
                println!("=== Phase 1 ===");
                println!("Choisir un Type d'ennemi.");
                println!("1. Gobelin");
                println!("2. Squelette");
                println!("3. Bandit");

                let enemy_type: EnemyType = loop {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();

                    match input.trim() {
                        "1" => break EnemyType::Gobelin,
                        "2" => break EnemyType::Skeleton,
                        "3" => break EnemyType::Human,
                        _ => println!("Choix invalide, recommence."),
                    }
                };

                // choisir le rang
                println!("=== Phase 2 ===");
                println!("Choisir un Rang d'ennemi.");
                println!("1. Lambda");
                println!("2. Nommé");
                println!("3. Elite");
                println!("4. Boss");
                println!("5. Légendaire");

                let enemy_rank: EnemyRank = loop {

                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();

                    match input.trim() {
                        "1" => break EnemyRank::Lambda,
                        "2" => break EnemyRank::Named,
                        "3" => break EnemyRank::Elite,
                        "4" => break EnemyRank::Boss,
                        "5" => break EnemyRank::Legendary,
                        _ => println!("Choix invalide, recommence."),
                    }
                };

                let mut enemy = Enemy::spawn_with_type_and_rank(enemy_type, enemy_rank);
                if let Some(name) = &enemy.name {
                    println!("Un {} apparaît ! Il s'agit de {}.", enemy.enemy_type.as_str(), name );
                }
                else {
                    println!("Un {} apparaît !", enemy.enemy_type.as_str());
                }

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
                    // gérer l'xp
                    player.gain_xp(enemy.rank.xp_reward());
                    state = GameState::Menu;
                }
            },
            GameState::GameOver => {
                println!("Tu es mort. Fin du jeu.");
                break;
            }
        }
    }
}

fn display_menu(){
    println!("=== MENU ===");
    println!("Bienvenue dans Little RPG un jeu écrit en Rust.");
    println!("Tapez :");
    println!("1. pour afficher ce Menu");
    println!("2. pour Combattre");
    println!("3. pour Quitter");
}