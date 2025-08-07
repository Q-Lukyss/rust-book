#[derive(Debug)]
pub enum Item {
    Potion(Potion),
    Weapon(Weapon),
    Shield,
}

#[derive(Debug, Clone)]
pub enum Potion {
    Healing(u32), // soin de X hp
    StatBoost { stat: Stat, value: u32, duration : u8 }, // augmente une stat
}

#[derive(Debug, Clone)]
pub enum WeaponKind {
    Sword,
    Axe,
    Spear,
    LongSword,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stat {
    Attack,
}

#[derive(Debug, Clone)]
pub struct Weapon {
    pub kind: WeaponKind,
    pub base_damage: u32,
}

#[derive(Debug, PartialEq)]

pub enum AttackPattern {
    EveryTurn,
    EveryTwoTurns,
    TwicePerTurn,
    EveryThreeTurns,
}

pub type Inventory = Vec<Item>;

impl WeaponKind {
    pub fn attack_pattern(&self) -> AttackPattern {
        match self {
            WeaponKind::Sword => AttackPattern::EveryTurn,
            WeaponKind::Axe => AttackPattern::EveryTwoTurns,
            WeaponKind::Spear => AttackPattern::TwicePerTurn,
            WeaponKind::LongSword => AttackPattern::EveryThreeTurns,
        }
    }
}

impl Weapon {
    pub fn get_dpt(&self) -> f32 {
        let pattern = self.kind.attack_pattern();
        let multiplier = match pattern {
            AttackPattern::EveryTurn => 1.0,
            AttackPattern::EveryTwoTurns => 0.5,
            AttackPattern::EveryThreeTurns => 1.0 / 3.0,
            AttackPattern::TwicePerTurn => 2.0,
        };
        self.base_damage as f32 * multiplier
    }

    pub fn display(&self) {
        println!(
            "{:?} | Dmg: {} | Pattern: {:?} | DPT: {:.2}",
            self.kind,
            self.base_damage,
            self.kind.attack_pattern(),
            self.get_dpt()
        );
    }
}