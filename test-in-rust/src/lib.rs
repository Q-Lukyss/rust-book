// https://doc.rust-lang.org/stable/book/ch11-00-testing.html

// cargo test
// cargo test -- --show-output -> permet de montrer les stdout comme println!
// cargo test <TEST_NAME> pour run un seul test
// pareil on peut preciser une partie du nom du test pour run tous les tests qui ont cette partie dans leur nom
// cargo test -- --ignored -> avec un d run les tests marqués du flag ignore sans d
// cargo test -- --include-ignored -> run tout 

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn greeting(name : &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
          if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {value}."
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {value}."
            );
        }

        Guess { value }
    }
}

// Tests Unitaire, dans le fichiers qu'ils testent avec un module de test mod tests
#[cfg(test)] // pour configuration test -> n'est run que pour cargo test
mod tests {
    use super::*; // permet d'utiliser tout ce qui est défni plus haut

    #[test]
    fn it_works() -> Result<(), String>{
        let result = add(2, 2);
        // avec Result<T, E>
         if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn larger_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn greeting_contains_argument() {
        let result = greeting("Lukyss");
        assert!(
            result.contains("Lukyss"),
            "Greeting ne contient pas la valeure name, valeure : {}", result
        );
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn guess_greater_than_100(){
        Guess::new(200);
    }

    #[test]
    #[ignore]
    fn fail(){
        panic!("ce test échoue")
    }
}