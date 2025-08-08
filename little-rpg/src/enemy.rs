use rand::Rng;
use rand::prelude::IndexedRandom; // Import nécessaire pour la méthode choose
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Deserialize)]
struct EnemyRow {
    r#type: EnemyType,
    rank: EnemyRank,
    names: Vec<String>,
}

#[derive(Deserialize)]
struct EnemyFile {
    enemies: Vec<EnemyRow>,
}

static ENEMY_DB: Lazy<Vec<EnemyRow>> = Lazy::new(|| {
    let p = Path::new("assets/enemies.toml");
    let s = fs::read_to_string(p).expect("assets/enemies.toml manquant");
    let f: EnemyFile = toml::from_str(&s).expect("TOML invalide");
    f.enemies
});

fn pick_name_from_db(
    t: EnemyType,
    r: EnemyRank,
    rng: &mut impl rand::Rng,
) -> Option<String> {
    ENEMY_DB.iter()
        .find(|e| e.r#type == t && e.rank == r)
        .and_then(|row| row.names.choose(rng).cloned())
}


#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, serde::Serialize, serde::Deserialize)]
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

    pub fn xp_reward(&self) -> u32 {
        match self {
            EnemyRank::Lambda => 20,
            EnemyRank::Named => 50,
            EnemyRank::Elite => 100,
            EnemyRank::Boss => 200,
            EnemyRank::Legendary => 500,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, serde::Serialize, serde::Deserialize)]
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
    pub fn spawn_with_type_and_rank(enemy_type: EnemyType, rank: EnemyRank) -> Self {
        let mut rng = rand::rng();

        let name = pick_name_from_db(enemy_type, rank, &mut rng);
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