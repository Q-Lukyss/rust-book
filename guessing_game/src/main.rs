use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Devinez le nombre!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {        
        println!("Entrez votre choix!");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Echec à lire la ligne");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Entrez nombre valide.");
                continue;
            }
        };

        println!("Vous avez deviné : {guess}");

        match guess.cmp(&secret_number)  {
            Ordering::Less => println!("Trop Petit!"),
            Ordering::Greater => println!("Trop Grand!"),
            Ordering::Equal => {
                println!("Victoire!");
                break;
            }
        }
    }


}
