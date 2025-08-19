use sails_rs::{calls::*, gtest::{calls::*, System}};

use vara_app_client::traits::*;

const ACTOR_ID: u64 = 42;

#[tokio::test]
async fn do_something_works() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");
    system.mint_to(ACTOR_ID, 100_000_000_000_000);
    let remoting = GTestRemoting::new(system, ACTOR_ID.into());

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(vara_app::WASM_BINARY);

    let program_factory = vara_app_client::VaraAppFactory::new(remoting.clone());

    let program_id = program_factory
        .new()
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = vara_app_client::VaraApp::new(remoting.clone());

    let program_id_balance = remoting.system().balance_of(program_id);
    println!("\n 1. balance: {program_id_balance:?} \n");

    let result = service_client
        .do_something(program_code_id)
        .with_value(20_000_000_000_000)
        .send_recv(program_id)
        .await
        .unwrap();

    let program_id_balance = remoting.system().balance_of(program_id);
    println!("\n 2. balance: {program_id_balance:?} \n");

    let new_program_id_balance = remoting.system().balance_of(result);
    println!("\n 3. balance: {new_program_id_balance:?} <- new program\n");

    assert_eq!(new_program_id_balance, 1000000000000);
    assert_eq!(program_id_balance,     20000000000000);
}
