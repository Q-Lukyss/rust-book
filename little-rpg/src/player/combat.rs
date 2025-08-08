use super::base::Player;
use crate::enemy::Enemy;
use crate::inventory::Weapon;

impl Player {
    fn player_attack(&self) -> i32 {
        self.attack + self.equiped_weapon.base_damage as i32
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

    pub fn equip_weapon(&mut self, weapon: Weapon) {
        println!("Tu équipes une nouvelle arme : {:?}, dmg : {}", weapon.kind, weapon.base_damage);
        self.equiped_weapon = weapon;
    }
}
