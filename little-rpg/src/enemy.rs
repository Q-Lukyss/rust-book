use rand::Rng;
use rand::prelude::IndexedRandom; // Import nécessaire pour la méthode choose
use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static ENEMY_NAMES: Lazy<HashMap<EnemyType, HashMap<EnemyRank, Vec<&'static str>>>> = Lazy::new(|| {
    HashMap::from([
        // Gobelins
        (EnemyType::Gobelin, HashMap::from([
            (EnemyRank::Named, vec![ "Gribz", "Zug", "Snark", "Grobnar", "Blix", "Druk", "Rikrik", "Glim", "Zoz", "Nark"]),
            (EnemyRank::Elite, vec!["Grum le Cruel", "Zkro le Methodiste", "Thrag le Brutal", "Makgorah l'Invaincu"]),
            (EnemyRank::Boss, vec!["Skarn le Destructeur", "Gorath l'ancien"]),
            (EnemyRank::Legendary, vec!["Gnorlax le père de toute choses", "Gshlomog la main d'or"]),
        ])),
        // Squelettes
        (EnemyType::Skeleton, HashMap::from([
            (EnemyRank::Named, vec!["Osram", "Clatter", "Drybone", "Rattle", "Marrow", "Skorn", "Bonelord", "Grimjaw", "Skarn", "Femur"]),
            (EnemyRank::Elite, vec!["Seigneur Fémur", "Capitaine Tibia", "Karg l’Osseux", "Morbius", ]),
            (EnemyRank::Boss, vec!["Archiliche Noctis"]),
            (EnemyRank::Legendary, vec!["Le Roi-Liche Eternel"]),
        ])),
        // Humains
        (EnemyType::Human, HashMap::from([
            (EnemyRank::Named, vec!["Silas", "Nyra", "Vex", "Kael", "Shade", "Lira", "Raze", "Dorin", "Thorne", "Nim"]),
            (EnemyRank::Elite, vec!["Valtor l’Ombre", "Sir Garrick", "Erik le Fourbe", "Kaelen le Vif"]),
            (EnemyRank::Boss, vec!["Commandant Draven", "Capitaine Valerius"]),
            (EnemyRank::Legendary, vec!["L’Empereur Noir"]),
        ])),
    ])
});


#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum EnemyRank {
    Lambda,
    Named,
    Elite,
    Boss,
    Legendary,
}

impl EnemyRank {
    fn stat_multiplier(&self) -> f32 {
        match self {
            EnemyRank::Lambda => 1.0,
            EnemyRank::Named => 1.5,
            EnemyRank::Elite => 5.0,
            EnemyRank::Boss => 3.0,
            EnemyRank::Legendary => 10.0,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum EnemyType {
    Gobelin,
    Skeleton,
    Human
}

impl EnemyType {
    pub fn as_str(&self) -> &str {
        match self {
            EnemyType::Gobelin => "Gobelin",
            EnemyType::Skeleton => "Squelette",
            EnemyType::Human => "Bandit",
        }
    }
}

pub struct Enemy {
    pub name: Option<String>,
    pub hp: i32,
    pub attack: i32,
    pub enemy_type: EnemyType,
    pub rank : EnemyRank
}

impl Enemy {
    pub fn spawn(enemy_type: EnemyType) -> Self {
        let mut rng = rand::rng();

        // Pondération : plus de chance d'avoir des lambdas
        let rank = match rng.random_range(0..100) {
            0..=70 => EnemyRank::Lambda,
            71..=90 => EnemyRank::Named,
            _ => EnemyRank::Elite,
        };

        Self::spawn_with_type_and_rank(enemy_type, rank)
    }


    pub fn spawn_multiple(enemy_type: EnemyType, count: usize) -> Vec<Self> {
        (0..count).map(|_| Self::spawn(enemy_type)).collect()
    }

    /// Spawne un boss avec une chance qu’il soit légendaire
    pub fn spawn_boss(enemy_type: EnemyType) -> Self {
        let mut rng = rand::rng();
        let rank = if rng.random_bool(0.2) {
            EnemyRank::Legendary
        } else {
            EnemyRank::Boss
        };

        Self::spawn_with_type_and_rank(enemy_type, rank)
    }

    /// Méthode utilitaire pour créer un ennemi à partir d'un type et rang donnés
    fn spawn_with_type_and_rank(enemy_type: EnemyType, rank: EnemyRank) -> Self {
        let mut rng = rand::rng();

        let name = ENEMY_NAMES
        .get(&enemy_type)
        .and_then(|inner| inner.get(&rank)) // récupère la liste de noms
        .and_then(|names| names.choose(&mut rng)) // pioche un nom aléatoire
        .map(|&s| s.to_string()); // transforme &str en String

        // Stats simples pour l’exemple
        let base_hp = 10;
        let base_attack = 3;

        Enemy {
            name, // Option<String>
            hp: (base_hp as f32 * rank.stat_multiplier()) as i32,
            attack: (base_attack as f32 * rank.stat_multiplier()) as i32,
            enemy_type,
            rank,
        }
    }
}