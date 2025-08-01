use std::io;

fn main() {
    println!("Calculer la valeur n de la suite de Fibonacci");
    println!("Entrez un nombre");

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Echec à lire la ligne");

    let number: i128 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Entrez nombre valide.");
            return
        }
    };

    let value = fib(number, 0, 1);

    println!("La valeur du {number}eme element de la suite de Fibonacci est {value}");        
}

fn fib(n : i128, a : i128, b : i128) -> i128 {
    if n == 0 {
        a
    }
    else if n == 1 {
        b
    }
    else {
        fib(n - 1, b, a + b)
    }
}
