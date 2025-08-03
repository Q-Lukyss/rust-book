use rand::Rng;
use rand::prelude::IndexedRandom; // Import nécessaire pour la méthode choose

pub const GOBELIN_NAMES: [&str; 10] = [
    "Gribz", "Zug", "Snark", "Grobnar", "Blix", "Druk", "Rikrik", "Glim", "Zoz", "Nark",
];

pub const VOLEUR_NAMES: [&str; 10] = [
    "Silas", "Nyra", "Vex", "Kael", "Shade", "Lira", "Raze", "Dorin", "Thorne", "Nim",
];

pub const SQUELETTE_NAMES: [&str; 10] = [
    "Osram", "Clatter", "Drybone", "Rattle", "Marrow", "Skorn", "Bonelord", "Grimjaw", "Skarn", "Femur",
];

pub enum EnemyType {
    Gobelin,
    Skeleton,
    Thief
}

impl EnemyType {
    pub fn as_str(&self) -> &str {
        match self {
            EnemyType::Gobelin => "Gobelin",
            EnemyType::Skeleton => "Squelette",
            EnemyType::Thief => "Voleur",
        }
    }
}

pub struct Enemy {
    pub name: String,
    pub hp: i32,
    pub attack: i32,
    pub enemy_type: EnemyType,
}

impl Enemy {
    pub fn spawn() -> Self {
        let mut rng = rand::rng();
        let n = rng.random_range(0..=2);

        match n {
            0 => Enemy {
                name: get_random_name(&EnemyType::Gobelin).to_string(), // Correction ici
                hp: 10,
                attack: 1,
                enemy_type: EnemyType::Gobelin,
            },
            1 => Enemy {
                name: get_random_name(&EnemyType::Skeleton).to_string(), // Correction ici
                hp: 8,
                attack: 2,
                enemy_type: EnemyType::Skeleton,
            },
            2 => Enemy {
                name: get_random_name(&EnemyType::Thief).to_string(), // Correction ici
                hp: 6,
                attack: 3,
                enemy_type: EnemyType::Thief,
            },
            _ => unreachable!(),
        }
    }
}

fn get_random_name(enemy_type: &EnemyType) -> &'static str {
    let mut rng = rand::rng();

    match enemy_type {
        EnemyType::Gobelin => GOBELIN_NAMES.choose(&mut rng).unwrap(), // Correction ici
        EnemyType::Thief => VOLEUR_NAMES.choose(&mut rng).unwrap(),   // Correction ici
        EnemyType::Skeleton => SQUELETTE_NAMES.choose(&mut rng).unwrap(), // Correction ici
    }
}