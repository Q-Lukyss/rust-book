// test d'intégration du point de vue d'un utilisateur de notre library
// cargo test -- --test integration_test pour tester ce fichier

use test_in_rust::add;

#[test]
fn it_adds(){
    let result = add(4, 6);
    assert_eq!(result, 10);
}