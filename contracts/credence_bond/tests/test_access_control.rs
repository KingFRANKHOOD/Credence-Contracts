use soroban_sdk::{testutils::Address as _, Address, Env, String};

use credence_bond::{CredenceBond, CredenceBondClient};

fn setup(env: &Env) -> (CredenceBondClient<'_>, Address, Address, Address) {
    env.mock_all_auths();

    let contract_id = env.register_contract(None, CredenceBond);
    let client = CredenceBondClient::new(env, &contract_id);

    let admin = Address::generate(env);
    let user = Address::generate(env);
    let attacker = Address::generate(env);

    client.initialize(&admin);

    (client, admin, user, attacker)
}

//
// ---------------- ATTESTER REGISTRATION ----------------
//

#[test]
fn anyone_can_register_attester_current_behavior() {
    let env = Env::default();
    let (client, _admin, _user, attacker) = setup(&env);

    // current contract allows this
    client.register_attester(&attacker);
}

//
// ---------------- ATTESTER RESTRICTIONS ----------------
//

#[test]
#[should_panic]
fn unauthorized_cannot_add_attestation() {
    let env = Env::default();
    let (client, _admin, user, attacker) = setup(&env);

    let fake = String::from_str(&env, "fake");
    client.add_attestation(&attacker, &user, &fake);
}

#[test]
fn authorized_attester_can_add_attestation() {
    let env = Env::default();
    let (client, _admin, user, attacker) = setup(&env);

    client.register_attester(&attacker);

    let valid = String::from_str(&env, "valid");
    let att = client.add_attestation(&attacker, &user, &valid);

    assert_eq!(att.subject, user);
}

#[test]
#[should_panic]
fn wrong_attester_cannot_revoke() {
    let env = Env::default();
    let (client, _admin, user, attacker) = setup(&env);

    client.register_attester(&attacker);

    let valid = String::from_str(&env, "valid");
    let att = client.add_attestation(&attacker, &user, &valid);

    let other = Address::generate(&env);
    client.revoke_attestation(&other, &att.id);
}

//
// ---------------- OWNER RESTRICTIONS ----------------
//

#[test]
#[should_panic]
fn non_owner_cannot_withdraw_bond() {
    let env = Env::default();
    let (client, _admin, user, attacker) = setup(&env);

    client.create_bond(&user, &1000, &100);
    client.withdraw_bond(&attacker);
}

#[test]
fn owner_can_withdraw_bond() {
    let env = Env::default();
    let (client, _admin, user, _) = setup(&env);

    client.create_bond(&user, &1000, &100);
    let amount = client.withdraw_bond(&user);

    assert_eq!(amount, 1000);
}
