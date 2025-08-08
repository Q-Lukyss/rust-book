use crate::inventory::{Inventory, Stat, Weapon};

pub struct Player {
    pub name: String,
    pub hp: (i32, i32),
    pub attack: i32,
    pub level: u32,
    pub xp: (u32, u32),
    pub inventory: Inventory,
    pub equiped_weapon: Weapon,
    pub buffs: Vec<ActiveBuff>,
}

pub struct ActiveBuff {
    pub stat: Stat,
    pub value: u32,
    pub remaining_turns: u8,
}
