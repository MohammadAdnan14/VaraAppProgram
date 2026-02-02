#![no_std]

use sails_rs::prelude::*;

struct NewAppService(());

impl NewAppService {
    pub fn new() -> Self {
        Self(())
    }
}

#[sails_rs::service]
impl NewAppService {
    // Service's method (command)
    #[export]
    pub fn do_something(&mut self) -> String {
        "Hello from NewApp!".to_string()
    }

    // Service's query
    #[export]
    pub fn get_something(&self) -> String {
        "Hello from NewApp!".to_string()
    }    
}

pub struct NewAppProgram(());

#[sails_rs::program]
impl NewAppProgram {
    // Program's constructor
    pub fn new() -> Self {
        Self(())
    }

    // Exposed service
    pub fn new_app(&self) -> NewAppService {
        NewAppService::new()
    }
}
