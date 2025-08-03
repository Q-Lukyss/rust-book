use crate::enemy::Enemy;
pub struct Player {
    pub name: String,
    pub hp: i32,
    pub attack: i32,
}
impl Player {
    pub fn attack(&self, enemy : &mut Enemy) {
        enemy.hp -= self.attack;
        if enemy.hp <= 0 {
            println!("le {} est mort !", enemy.enemy_type.as_str());
        }
    }

    pub fn take_dmg(&mut self, amount : i32){
        self.hp -= amount;
        println!("Il te reste {} hp",self.hp);
    }

    pub fn new(name: String) -> Self {
        Self {
            name,
            hp: 20,
            attack: 5,
        }
    }
}
