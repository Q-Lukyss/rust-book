use crate::state::GameState;
use crate::player::Player;
use crate::enemy::{Enemy, EnemyRank, EnemyType};
use std::io::{self, Write};
use crate::inventory::{WeaponKind, AttackPattern, Item, Potion};

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
                    "2" => state = GameState::Exploration,
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

                let enemy = Enemy::spawn_with_type_and_rank(enemy_type, enemy_rank);
                if let Some(name) = &enemy.name {
                    println!("Un {} apparaît ! Il s'agit de {}.", enemy.enemy_type.as_str(), name );
                }
                else {
                    println!("Un {} apparaît !", enemy.enemy_type.as_str());
                }

                state = GameState::Combat(enemy)
            },
            GameState::Combat(mut enemy) => {
                println!("{}, tu entre en combat...", player.name);

                let mut turn = 1;
                let mut next_state = None;

                while enemy.hp > 0 && player.hp.0 > 0  && next_state.is_none(){
                    player.update_buffs();

                    println!("\n=== Combat ===");
                    println!("Tes PV : {}/{}", player.hp.0, player.hp.1);
                    println!("PV de l'ennemi : {}", enemy.hp);
                    println!("Que veux-tu faire ?");
                    println!("1. Attaquer");
                    println!("2. Utiliser un objet");
                    println!("3. Fuir");

                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();

                    match input.trim() {
                        "1" => {
                            let pattern = player.equiped_weapon.kind.attack_pattern();

                            if pattern == AttackPattern::TwicePerTurn {
                                println!("Attaque 1 !");
                                player.attack(&mut enemy);
                                if enemy.hp > 0 {
                                    println!("Attaque 2 !");
                                    player.attack(&mut enemy);
                                }
                            } else if can_attack_this_turn(turn, &player.equiped_weapon.kind) {
                                println!("Tu attaques !");
                                player.attack(&mut enemy);
                            } else {
                                println!("Ton arme ne peut pas attaquer ce tour !");
                            }

                            if enemy.hp > 0 {
                                println!("L'ennemi riposte !");
                                player.take_dmg(enemy.attack);
                            }
                        },
                        "2" => {
                            let potions: Vec<(usize, &Item)> = player.inventory.iter()
                                .enumerate()
                                .filter(|(_, item)| matches!(item, Item::Potion(_)))
                                .collect();

                            if potions.is_empty() {
                                println!("Tu n'as aucune potion.");
                            } else {
                                println!("=== Potions disponibles ===");
                                for (i, item) in &potions {
                                    println!("{}. {:?}", i + 1, item);
                                }

                                println!("Choisis une potion (ou tape autre chose pour annuler) :");
                                let mut item_input = String::new();
                                io::stdin().read_line(&mut item_input).unwrap();

                                if let Ok(index) = item_input.trim().parse::<usize>() {
                                    if let Some((real_index, Item::Potion(potion))) = potions.into_iter().find(|(i, _)| *i + 1 == index) {
                                        // utilise la potion
                                        player.use_potion(potion.clone());
                                        player.inventory.remove(real_index);
                                    } else {
                                        println!("Potion invalide.");
                                    }
                                }
                            }
                        },
                        "3" => {
                            println!("Tu prends la fuite !");
                            next_state = Some(GameState::Exploration);
                        }
                        _ => {
                            println!("Commande invalide !");
                        }
                    };

                    turn += 1; //incrément du tour
                }
                    
                if let Some(new_state) = next_state {
                    state = new_state;
                } else if player.hp.0 <= 0 {
                    state = GameState::GameOver;
                } else {
                    player.gain_xp(enemy.rank.xp_reward());
                    state = GameState::Menu;
                }
        
            },
            GameState::Inventory => {
                println!("=== Gestion de l'inventaire ===");
                println!("1. Voir les armes");
                println!("2. Retour");

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                match input.trim() {
                    "1" => {
                        let weapons: Vec<(usize, &Item)> = player.inventory.iter()
                            .enumerate()
                            .filter(|(_, item)| matches!(item, Item::Weapon(_)))
                            .collect();

                        if weapons.is_empty() {
                            println!("Aucune arme dans l'inventaire.");
                        } else {
                            println!("=== Armes ===");
                            for (i, item) in &weapons {
                                if let Item::Weapon(w) = item {
                                    println!("{}. {:?} - dmg: {}", i + 1, w.kind, w.base_damage);
                                }
                            }

                            println!("Quelle arme veux-tu équiper ?");
                            let mut choice = String::new();
                            io::stdin().read_line(&mut choice).unwrap();

                            if let Ok(index) = choice.trim().parse::<usize>() {
                                if let Some((real_index, Item::Weapon(w))) = weapons.into_iter().find(|(i, _)| *i + 1 == index) {
                                    player.equip_weapon(w.clone());
                                    player.inventory.remove(real_index);
                                }
                            }
                        }
                    },
                    "2" => state = GameState::Menu,
                    _ => println!("Choix invalide."),
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

fn can_attack_this_turn(turn: u32, weapon_kind: &WeaponKind) -> bool {
    match weapon_kind.attack_pattern() {
        AttackPattern::EveryTurn => true,
        AttackPattern::EveryTwoTurns => turn % 2 == 0,
        AttackPattern::EveryThreeTurns => turn % 3 == 0,
        AttackPattern::TwicePerTurn => true
    }
}