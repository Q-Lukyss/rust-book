use crate::enemy::Enemy;
use crate::inventory::{Inventory, Item, Potion, Stat, Weapon, WeaponKind};

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

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name,
            hp: (20, 20),
            attack: 0,
            level: 1,
            xp: (0, 100),
            inventory: vec![Item::Potion(Potion::Healing(10))], // potion initiale
            equiped_weapon: Weapon {
                kind: WeaponKind::Sword,
                base_damage: 5,
            },
            buffs: vec![],
        }
    }

    fn player_attack(&self) -> i32 {
        self.attack + self.equiped_weapon.base_damage as i32
    }

    fn set_xp_to_level_up(&mut self) {
        self.xp.1 = self.level * 100;
    }

    fn display_xp(&self) {
        println!("Xp : {} / {}", self.xp.0, self.xp.1);
    }

    pub fn use_potion(&mut self, potion: Potion) {
        match potion {
            Potion::Healing(amount) => {
                self.hp.0 = (self.hp.0 + amount as i32).min(self.hp.1);
                println!("Tu récupères {} PV ! ({}/{})", amount, self.hp.0, self.hp.1);
            }
            Potion::StatBoost {
                stat,
                value,
                duration,
            } => {
                println!("Ton/ta {:?} augmente de {} pour {} tours !", stat, value, duration);
                self.buffs.push(ActiveBuff {
                    stat: stat.clone(),
                    value,
                    remaining_turns: duration,
                });

                if stat == Stat::Attack {
                    self.attack += value as i32;
                }
            }
        }
    }

    pub fn attack(&self, enemy: &mut Enemy) -> bool {
        enemy.hp -= self.player_attack();
        if enemy.hp <= 0 {
            println!("Le {} est mort !", enemy.enemy_type.as_str());
            return true;
        }
        false
    }

    pub fn take_dmg(&mut self, amount: i32) {
        self.hp.0 -= amount;
        if self.hp.0 <= 0 {
            self.hp.0 = 0;
        }
        println!("Il te reste {} hp", self.hp.0);
    }

    pub fn gain_xp(&mut self, amount: u32) {
        println!("Vous avez gagné {} points d'xp", amount);
        self.xp.0 += amount;
        self.display_xp();
        let xp_to_level_up = self.xp.1;

        while self.xp.0 >= xp_to_level_up {
            self.xp.0 -= xp_to_level_up;
            self.level_up();
        }
    }

    pub fn level_up(&mut self) {
        self.level += 1;
        self.set_xp_to_level_up();
        self.hp.1 += 1;
        self.hp.0 = self.hp.1;
        self.attack += 1;
        println!("{} a gagné un niveau !", self.name);
        println!("Toutes les stats ont pris +1");
        println!("PV soignés au max {}", self.hp.1);
        self.display_xp();
    }

    pub fn equip_weapon(&mut self, weapon: Weapon) {
        println!(
            "Tu équipes une nouvelle arme : {:?}, dmg : {}",
            weapon.kind, weapon.base_damage
        );
        self.equiped_weapon = weapon;
    }

    pub fn update_buffs(&mut self) {
        let mut expired = vec![];

        for (i, buff) in self.buffs.iter_mut().enumerate() {
            if buff.remaining_turns > 0 {
                buff.remaining_turns -= 1;
            }

            if buff.remaining_turns == 0 {
                println!("L'effet du boost sur {:?} a expiré.", buff.stat);
                if buff.stat == Stat::Attack {
                    self.attack = self.attack.saturating_sub(buff.value as i32);
                }
                expired.push(i);
            }
        }

        for &index in expired.iter().rev() {
            self.buffs.remove(index);
        }
    }
}
