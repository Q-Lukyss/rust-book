use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{fs, path::Path};

use crate::enemy::{EnemyRank, EnemyType};

#[derive(Deserialize)]
pub struct EnemyRow {
    #[serde(rename = "type")]
    pub kind: EnemyType,
    pub rank: EnemyRank,
    pub names: Vec<String>,
}

#[derive(Deserialize)]
struct EnemyFile {
    enemies: Vec<EnemyRow>,
}

// Charge assets/enemies.toml une seule fois
pub static ENEMY_DB: Lazy<Vec<EnemyRow>> = Lazy::new(|| {
    let p = Path::new("assets/enemies.toml");
    let s = fs::read_to_string(p).expect("assets/enemies.toml manquant");
    let f: EnemyFile = toml::from_str(&s).expect("TOML invalide");
    f.enemies
});

/// Retourne un nom aléatoire ou None si non trouvé
pub fn pick_name_from_db(
    t: EnemyType,
    r: EnemyRank,
    rng: &mut impl rand::Rng,
) -> Option<String> {
    // rand 0.9 -> IndexedRandom ; rand 0.8 -> SliceRandom
    use rand::prelude::IndexedRandom;
    ENEMY_DB
        .iter()
        .find(|e| e.kind == t && e.rank == r)
        .and_then(|row| row.names.choose(rng).cloned())
}
