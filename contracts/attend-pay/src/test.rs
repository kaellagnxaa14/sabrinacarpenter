#![cfg(test)]

use soroban_sdk::{testutils::Address as _, BytesN, Env, Address};
use crate::{AttendPayContract, AttendPayContractClient};

#[test]
fn test_happy_path_attendance_and_reward() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AttendPayContract);
    let client = AttendPayContractClient::new(&env, &contract_id);

    let student = Address::generate(&env);
    let date = BytesN::from_array(&env, &[1; 32]);

    client.mark_attendance(&student, &date);
    client.reward_student(&student);

    assert!(client.check_attendance(&student, &date));
}

#[test]
#[should_panic]
fn test_duplicate_attendance() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AttendPayContract);
    let client = AttendPayContractClient::new(&env, &contract_id);

    let student = Address::generate(&env);
    let date = BytesN::from_array(&env, &[2; 32]);

    client.mark_attendance(&student, &date);
    client.mark_attendance(&student, &date); // fail
}

#[test]
fn test_state_verification() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AttendPayContract);
    let client = AttendPayContractClient::new(&env, &contract_id);

    let student = Address::generate(&env);
    let date = BytesN::from_array(&env, &[3; 32]);

    client.mark_attendance(&student, &date);

    let result = client.check_attendance(&student, &date);
    assert_eq!(result, true);
}