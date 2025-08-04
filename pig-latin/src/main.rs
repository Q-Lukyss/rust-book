// regle convertir une string
// les premiere consonnes sont bougé + ay -> know  : ow-knay
// si le mot commence par une voyelle on ajoute hay -> owl : owl-hay

fn main() {
    // on veut prendre chaque item
    // si voyelle -> opération voyelle
    // si consonne -> on stocke, indice d'apres 
    // si consonne on ajoute au vector, indice suivant
    // si voyelle on fait opération consonne
    let data = vec!["apple", "know", "owl", "pain", "admonishement", "president", "resident"];

    let new_data : Vec<String> = data
        .iter()
        .map(|x| to_pig_latin(x))
        .collect();

    println!("Liste des mots en piglatin {:?}", new_data);
}

fn to_pig_latin(word : &str) -> String {
    
    if is_first_voyelle(word) {
        format!("{word}-hay")
    }
    else {
        let mut first_voyelle_index = word.len();
        for (i, c) in word.char_indices() {
            if "aeiouy".contains(c) {
                first_voyelle_index = i;
                break;
            }
        }

        let (consonne, reste) = word.split_at(first_voyelle_index);
        format!("{reste}-{consonne}ay")
    }
}

fn is_first_voyelle(word: &str) -> bool {
    let voyelles = "aeiouy";
    word.chars()
        .next()
        .map(|c| voyelles.contains(c))
        .unwrap_or(false)
}
