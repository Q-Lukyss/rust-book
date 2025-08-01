use std::io;

fn main() {
    println!("Convertisseur Température");
    println!("Tapez C pour convertir Celsius en Fahrenheit ou F pour Fahrenheit en Celsius");

    let mut choix = String::new();

        io::stdin()
            .read_line(&mut choix)
            .expect("Echec à lire la ligne");

        println!("{}", choix.to_lowercase());

        if choix.trim().to_lowercase() == "c" {
            let mut choix_c = String::new();
            println!("Entrez votre valeure en Celsius");

            io::stdin()
            .read_line(&mut choix_c)
            .expect("Echec à lire la ligne");

            let choix_c: f64 = match choix_c.trim().parse() {
                Ok(valeur) => valeur,
                Err(_) => {
                    println!("Erreur de parsing : veuillez entrer un nombre valide.");
                    return;
                }
            };

            celsius_to_fahrenheit(choix_c)
        }
        else if choix.trim().to_lowercase() == "f" {
            let mut choix_f = String::new();
            println!("Entrez votre valeure en Fahrenheit");

            io::stdin()
            .read_line(&mut choix_f)
            .expect("Echec à lire la ligne");

             let choix_f: f64 = match choix_f.trim().parse() {
                Ok(valeur) => valeur,
                Err(_) => {
                    println!("Erreur de parsing : veuillez entrer un nombre valide.");
                    return;
                }
            };

            fahrenheit_to_celsius(choix_f)
        }
        else {
            println!("Choix invalide");
        }
        
        

}

fn fahrenheit_to_celsius(degree_f : f64) {
    let degree_c : f64 = (degree_f - 32.0) * 5.0/9.0;
    println!("{degree_f}°F valent {degree_c}°C");
}

fn celsius_to_fahrenheit(degree_c : f64) {
    let degree_f : f64 = degree_c * 9.0/5.0 +32.0;
    println!("{degree_c}°C valent {degree_f}°F");
}