#![cfg_attr(not(feature = "export-abi"), no_std)]

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use stylus_sdk::{
    prelude::*,
    alloy_primitives::{U256, Address},
};

sol_storage! {
    #[entrypoint]
    pub struct Contract {
        uint256 number;
        mapping(address => uint256) balances;
        uint256[] numbers;
    }
}

#[public]
impl Contract {
    // 1. 비효율적인 스토리지 접근
    pub fn get_balance(&self, account: Address) -> Result<U256, Vec<u8>> {
        Ok(self.balances.get(account))
    }

    // 2. 비효율적인 배열 접근
    pub fn get_number_at(&self, index: u32) -> Result<U256, Vec<u8>> {
        if index >= self.numbers.len() {
            return Err(vec![]);
        }
        Ok(self.numbers[index])
    }

    // 3. 비효율적인 상태 업데이트
    pub fn transfer(&mut self, to: Address, amount: U256) -> Result<(), Vec<u8>> {
        let from_balance = self.balances.get(msg::sender());
        if from_balance < amount {
            return Err(vec![]);
        }
        
        // 여러 번의 스토리지 접근
        self.balances.set(msg::sender(), from_balance - amount);
        let to_balance = self.balances.get(to);
        self.balances.set(to, to_balance + amount);
        
        Ok(())
    }

    // 4. 비효율적인 배열 조작
    pub fn add_number(&mut self, number: U256) -> Result<(), Vec<u8>> {
        self.numbers.push(number);
        Ok(())
    }

    // 5. 비효율적인 메모리 사용
    pub fn get_all_numbers(&self) -> Result<Vec<U256>, Vec<u8>> {
        let mut result = Vec::new();
        for i in 0..self.numbers.len() {
            result.push(self.numbers[i]);
        }
        Ok(result)
    }
} 