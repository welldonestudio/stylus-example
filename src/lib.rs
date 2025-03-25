#![cfg_attr(not(feature = "export-abi"), no_std)]

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use stylus_sdk::{
    prelude::*,
    alloy_primitives::U256,
};

sol_storage! {
    #[entrypoint]
    pub struct Contract {
        uint256 number;
    }
}

#[public]
impl Contract {
    pub fn constructor() -> Result<(), Vec<u8>> {
        Ok(())
    }

    pub fn get_number(&self) -> Result<U256, Vec<u8>> {
        Ok(self.number.get())
    }

    pub fn set_number(&mut self, new_number: U256) -> Result<(), Vec<u8>> {
        self.number.set(new_number);
        Ok(())
    }
}
