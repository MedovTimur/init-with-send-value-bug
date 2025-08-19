#![no_std]

use sails_rs::prelude::*;
use sails_rs::gstd::msg;

struct VaraAppService(());

#[sails_rs::service]
impl VaraAppService {
    pub fn new() -> Self {
        msg::send_with_gas(msg::source(), (), 0, msg::value()).expect("Error during send value");
        Self(())
    }

    // Service's method (command)
    pub fn do_something(&mut self) -> String {
        msg::send_with_gas(msg::source(), (), 0, msg::value()).expect("Error during send value");
        "Hello from VaraApp!".to_string()
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
        Self(())
    }

    // Exposed service
    pub fn vara_app(&self) -> VaraAppService {
        VaraAppService::new()
    }
}
