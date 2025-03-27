#![cfg_attr(not(feature = "export-abi"), no_std)]

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use stylus_sdk::{
    prelude::*,
    alloy_primitives::{U256, Address},
    storage::{StorageU256, StorageMap},
    msg,
    host::VM,
};

sol_storage! {
    #[entrypoint]
    pub struct Contract {
        #[borrow]
        __stylus_host: Host,
        uint256 number;
        mapping(address => uint256) balances;
        uint256[] numbers;
    }
}

impl StorageType for Contract {
    type Wraps<'a> = Contract;
    type WrapsMut<'a> = Contract;

    unsafe fn new(slot: U256, offset: u8, host: VM) -> Self {
        Self {
            number: StorageU256::new(slot, offset, host.clone()),
            balances: StorageMap::new(slot, offset + 1, host),
            numbers: Vec::new(),
        }
    }

    fn load<'a>(self) -> Self::Wraps<'a> {
        self
    }

    fn load_mut<'a>(self) -> Self::WrapsMut<'a> {
        self
    }
}

impl ValueDenier for Contract {
    fn deny_value(&self, _: &str) -> Result<(), Vec<u8>> {
        Ok(())
    }
}

impl HostAccess for Contract {
    fn vm(&self) -> &dyn Host {
        unsafe { &*VM::instance() }
    }
}

#[public]
impl Contract {
    pub fn constructor() {
        unsafe {
            let sender = msg::sender();
            let mut contract = Contract {
                number: StorageU256::new(U256::ZERO, 0, VM::instance()),
                balances: StorageMap::new(U256::ZERO, 1, VM::instance()),
                numbers: Vec::new(),
            };
            contract.number.set(U256::ZERO);
        }
    }

    pub fn get_number(&self) -> U256 {
        self.number.get()
    }

    pub fn set_number(&mut self, new_number: U256) {
        self.number.set(new_number);
    }

    pub fn get_balance(&self, account: Address) -> Result<U256, Vec<u8>> {
        Ok(self.balances.get(account))
    }

    pub fn get_number_at(&self, index: u32) -> Result<U256, Vec<u8>> {
        if index >= self.numbers.len() {
            return Err(vec![]);
        }
        Ok(self.numbers[index])
    }

    pub fn deposit(&mut self) {
        let sender = msg::sender();
        let value = msg::value();
        let current_balance = self.get_balance(sender)?;
        self.balances.insert(sender, current_balance + value);
    }

    pub fn withdraw(&mut self, amount: U256) {
        let sender = msg::sender();
        let current_balance = self.get_balance(sender)?;
        assert!(current_balance >= amount, "Insufficient balance");
        self.balances.insert(sender, current_balance - amount);
        unsafe {
            self.vm().call(sender, &[], amount).unwrap();
        }
    }

    pub fn transfer(&mut self, to: Address, amount: U256) -> Result<(), Vec<u8>> {
        let from_balance = self.balances.get(msg::sender())?;
        if from_balance < amount {
            return Err(vec![]);
        }
        
        self.balances.set(msg::sender(), from_balance - amount);
        self.balances.set(to, self.balances.get(to)? + amount);
        
        Ok(())
    }

    pub fn add_number(&mut self, number: U256) -> Result<(), Vec<u8>> {
        self.numbers.push(number);
        Ok(())
    }

    pub fn calculate_compound_interest(&mut self, principal: U256, rate: U256, time: U256) -> Result<U256, Vec<u8>> {
        let rate_decimal = rate.checked_div(U256::from(100)).unwrap_or(U256::ZERO);
        
        let mut compound = U256::from(1);
        let base = U256::from(1) + rate_decimal;
        
        let mut remaining_time = time;
        while remaining_time > U256::ZERO {
            compound = compound.checked_mul(base).unwrap_or(U256::ZERO);
            remaining_time = remaining_time.checked_sub(U256::from(1)).unwrap_or(U256::ZERO);
        }
        
        Ok(principal.checked_mul(compound).unwrap_or(U256::ZERO))
    }
}
