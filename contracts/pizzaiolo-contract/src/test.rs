#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_submit_and_get_score() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(PizzaioloContract, ());
    let client = PizzaioloContractClient::new(&env, &contract_id);

    let user = Address::generate(&env);

    client.submit_score(&user, &500);
    assert_eq!(client.get_score(&user), 500);

    // No debería bajar el puntaje si enviamos uno menor
    client.submit_score(&user, &300);
    assert_eq!(client.get_score(&user), 500);

    // Debería subir si enviamos uno mayor
    client.submit_score(&user, &1000);
    assert_eq!(client.get_score(&user), 1000);
}
