use std::collections::HashMap;

fn main() {
    let mut data = vec![86,4861,1465,61,615,5,641,46185,64,74,641,6481,6841,651,74,51,64,8641,6841,44,8461,8,486];
    data.sort();

    let mediane = data[data.len() / 2];

    let mut tableau  = HashMap::new(); 

    for value in &data  {
        let count = tableau.entry(value).or_insert(0);
        *count += 1
    }

    let max_count = tableau.values().copied().max().unwrap_or(0);

    // Récupérer toutes les valeurs qui apparaissent autant de fois
    let valeurs_frequentes: Vec<_> = tableau
        .iter()
        .filter(|&(_, &count)| count == max_count)
        .map(|(&val, _)| *val)
        .collect();

    // Affichage
    println!("Valeurs triées : {data:?}");
    println!("La médiane est : {mediane}");
    println!("Valeur(s) la/les plus fréquente(s) : {valeurs_frequentes:?} (apparition : {max_count} fois)");
    
}
