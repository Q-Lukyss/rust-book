// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company; 
// for example, “Add Sally to Engineering” or “Add Amir to Sales.” 
// Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
use std::collections::HashMap;
use std::io;

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    populate(&mut company);

    // boucle
    // on dit bienvenue et on demanda a l'utilisateur ce qu'il veut faire
    // List All, List <Departement>, Add, Exit
    println!("Bienvenu dans le Gestionnaire d'Entreprise");
    menu();
    
    loop {
        let mut choix = String::new();

        io::stdin()
            .read_line(&mut choix)
            .expect("Echec à lire la ligne");

        let input = choix.trim().to_lowercase();

        println!("Votre choix : {}.", input);

        let parts: Vec<&str> = input.split_whitespace().collect();

        match parts.as_slice() {
            ["exit"] => {
                println!("Au revoir !");
                break;
            }
            ["menu"] => menu(),
            ["list", "all"] => list_all(&company),
            ["list", dept] => list(&company, dept),
            ["add", name, "to", dept] => add(&mut company, name, dept),
            _ => println!("Commande inconnue ou mal formée."),
        }
    }
}

fn list_all(company: &HashMap<String, Vec<String>>) {
    let mut departments: Vec<_> = company.iter().collect();
    departments.sort_by_key(|(dept, _)| dept.to_lowercase());

    if departments.is_empty() {
        println!("L'entreprise n'as pas encore de département ni de personnel");
    }
    else {
        for (dept, employees) in departments {
            println!("📂 Département : {}", dept);

            let mut sorted_employees = employees.clone();
            sorted_employees.sort();

            for name in sorted_employees {
                println!("  👤 {}", name);
            }
        }   
    }
}

fn list(company: &HashMap<String, Vec<String>>, department : &str){
    // trier tous les employes du secteur par ordre alphabetic
    match company.get(department){
        Some(employe) if !employe.is_empty() => {
            let mut sorted = employe.clone();
            sorted.sort();

            println!("{} employés dans le département {}.", sorted.len(), department);
            for employe in sorted {
                println!("👤 {}", employe);
            }
        }
        Some(_) => {
            println!("Aucun employé dans le département {}.", department);
        }
        None => {
            println!("Le Département {} n'existe pas.", department);
        }
    }

}

fn add(company : &mut HashMap<String, Vec<String>>, name : &str, department : &str){
    // ajouter un employé au service
    let dept_key = department.to_lowercase(); // normalisation
    let emp_name = name.to_string(); // conversion explicite

    let employees = company.entry(dept_key).or_insert_with(Vec::new);

    if !employees.contains(&emp_name) {
        employees.push(emp_name);
        employees.sort(); // garde la liste triée
        println!("Employé ajouté !");
    } else {
        println!("Cet employé est déjà dans ce département.");
    }
}

fn menu(){
    println!("==== Menu ====");
    println!("List All pour lister les employés par Secteur.");
    println!("List <Nom du Secteur> pour lister les employés du secteur.");
    println!("Add <Prénom Employé> to <Secteur> pour ajouter un employé au secteur.");
    println!("Menu pour afficher cette aide.");
    println!("Exit pour quitter le programme.");
}

fn populate(company: &mut HashMap<String, Vec<String>>) {
    company
        .entry("Developpement".to_string().to_lowercase())
        .or_insert_with(Vec::new)
        .extend(vec!["Quentin".to_string(), "Jean".to_string(), "Vincent".to_string()]);

    company
        .entry("Achats".to_string().to_lowercase())
        .or_insert_with(Vec::new)
        .push("Amir".to_string());

    company
        .entry("RH".to_string().to_lowercase())
        .or_insert_with(Vec::new)
        .extend(vec!["Alice".to_string(), "Bruno".to_string()]);
}