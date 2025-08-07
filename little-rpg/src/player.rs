use crate::enemy::Enemy;
pub struct Player {
    pub name: String,
    pub hp: (i32, i32),
    pub attack: i32,
    pub level: u32,
    pub xp : (u32, u32)
}
impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name,
            hp: (20,20),
            attack: 5,
            level: 1,
            xp : (0 , 100)
        }
    }

    fn set_xp_to_level_up(&mut self) {
        self.xp.1 = self.level * 100
    }

    pub fn attack(&self, enemy: &mut Enemy) -> bool {
        enemy.hp -= self.attack;
        if enemy.hp <= 0 {
            println!("Le {} est mort !", enemy.enemy_type.as_str());
            return true;
        }
        false
    }

    pub fn take_dmg(&mut self, amount : i32){
        self.hp.0 -= amount;
        if self.hp.0 <= 0 {
            self.hp.0 = 0
        }
        println!("Il te reste {} hp",self.hp.0);
    }

    pub fn gain_xp(&mut self, amount : u32){
        println!("Vous avez gagné {} points d'xp", amount);
        self.xp.0 += amount;
        let xp_to_level_up = self.xp.1;

        while self.xp.0 >= xp_to_level_up {
            self.xp.0 -= xp_to_level_up;
            self.level_up();
        }
    }

    pub fn level_up(&mut self){
        // on gagne un lv
        self.level += 1;
        self.set_xp_to_level_up();
        // on se heal max life et on gagne 1 pv
        self.hp.1 += 1;
        self.hp.0 = self.hp.1;
        // on gagne + 1 attack
        self.attack += 1;
        println!("{} a gagné un niveau !", self.name);
        println!("Toutes les stats ont pris +1");
        println!("PV soignés au max {}", self.hp.1);
        println!("Xp : {} / {}", self.xp.0, self.xp.1);
    }

    
}
