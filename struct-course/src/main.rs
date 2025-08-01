// Idee exo transformer ce programme pour qu'il demande a l'utilisateur ses infos pour créer un utilisateur

fn main() {
    let person1 = Person {
        firstname: String::from("John"),
        lastname: String::from("Doe"),
        age: 18,
    };

    person1.is_majeur();
}

struct Person {
    firstname: String,
    lastname: String,
    age: u8, // max 255 suffisant pour un age
}

impl Person {
    fn is_majeur_in_europe(&self) -> String {
        if self.age >= 18 {
            format!("{} {} est majeur en Europe", self.firstname, self.lastname)
        } else {
            format!("{} {} n'est PAS majeur en Europe", self.firstname, self.lastname)
        }
    }

    fn is_majeur_in_america(&self) -> String {
        if self.age >= 21 {
            format!("{} {} est majeur aux États-Unis", self.firstname, self.lastname)
        } else {
            format!("{} {} n'est PAS majeur aux États-Unis", self.firstname, self.lastname)
        }
    }

    fn is_majeur(&self) {
        println!("{} {} a {} ans", self.firstname, self.lastname, self.age);
        println!("{}", self.is_majeur_in_europe());
        println!("{}", self.is_majeur_in_america());
    }
}
