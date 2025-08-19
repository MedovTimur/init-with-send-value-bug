#![no_std]

use sails_rs::prelude::*;
use sails_rs::gstd::msg;
use gstd::prog::ProgramGenerator;
use sails_rs::gstd::exec;

struct VaraAppService(());

impl VaraAppService {
    pub fn init() -> Self {
        msg::send_with_gas(msg::source(), (), 0, msg::value()).expect("Error during send value");
        Self(())
    }
}
#[sails_rs::service]
impl VaraAppService {
    pub fn new() -> Self {
        Self(())
    }

    // Service's method (command)
    pub async fn do_something(&mut self, code_id: CodeId) -> ActorId {
        let create_program_future = ProgramGenerator::create_program_bytes_with_gas_for_reply(
            code_id,
            ("New",  ()).encode(),
            5_000_000_000,
            msg::value() - exec::env_vars().existential_deposit,
            0,
        )
        .expect("Error ProgramGenerator");

        let (address, _) = create_program_future.await.expect("Error to create program");
        address
    }

    // Service's query
    pub fn get_something(&self) -> String {
        "Hello from VaraApp!".to_string()
    }    
}

pub struct VaraAppProgram(());

#[sails_rs::program]
impl VaraAppProgram {
    // Program's constructor
    pub fn new() -> Self {
        VaraAppService::init();
        Self(())
    }

    // Exposed service
    pub fn vara_app(&self) -> VaraAppService {
        VaraAppService::new()
    }
}
