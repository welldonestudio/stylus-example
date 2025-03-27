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
    // 1. 최적화된 스토리지 접근 (캐시 사용)
    pub fn get_balance(&self, account: Address) -> Result<U256, Vec<u8>> {
        // 0.8.3에서는 스토리지 접근이 캐시됩니다
        Ok(self.balances.get(account))
    }

    // 2. 최적화된 배열 접근 (바운드 체크 최적화)
    pub fn get_number_at(&self, index: u32) -> Result<U256, Vec<u8>> {
        // 0.8.3에서는 바운드 체크가 최적화됩니다
        if index >= self.numbers.len() {
            return Err(vec![]);
        }
        Ok(self.numbers[index])
    }

    // 3. 최적화된 상태 업데이트 (배치 처리)
    pub fn transfer(&mut self, to: Address, amount: U256) -> Result<(), Vec<u8>> {
        let from_balance = self.balances.get(msg::sender());
        if from_balance < amount {
            return Err(vec![]);
        }
        
        // 0.8.3에서는 스토리지 업데이트가 배치 처리됩니다
        self.balances.set(msg::sender(), from_balance - amount);
        self.balances.set(to, self.balances.get(to) + amount);
        
        Ok(())
    }

    // 4. 최적화된 배열 조작 (메모리 할당 최적화)
    pub fn add_number(&mut self, number: U256) -> Result<(), Vec<u8>> {
        // 0.8.3에서는 배열 조작이 최적화됩니다
        self.numbers.push(number);
        Ok(())
    }

    // 5. 최적화된 메모리 사용 (고정 크기 배열)
    pub fn get_all_numbers(&self) -> Result<Vec<U256>, Vec<u8>> {
        // 0.8.3에서는 고정 크기 배열을 사용할 수 있습니다
        let len = self.numbers.len();
        let mut result = Vec::with_capacity(len);
        for i in 0..len {
            result.push(self.numbers[i]);
        }
        Ok(result)
    }

    // 6. 새로운 최적화 기능: 배치 전송
    pub fn batch_transfer(&mut self, recipients: &[Address], amounts: &[U256]) -> Result<(), Vec<u8>> {
        if recipients.len() != amounts.len() {
            return Err(vec![]);
        }

        let sender = msg::sender();
        let mut total = U256::ZERO;
        
        // 먼저 총액 계산
        for amount in amounts {
            total += *amount;
        }

        // 잔액 확인
        let balance = self.balances.get(sender);
        if balance < total {
            return Err(vec![]);
        }

        // 배치 처리로 가스비 절감
        self.balances.set(sender, balance - total);
        
        for (recipient, amount) in recipients.iter().zip(amounts.iter()) {
            self.balances.set(*recipient, self.balances.get(*recipient) + *amount);
        }

        Ok(())
    }

    // 7. 새로운 최적화 기능: 메모리 캐시
    pub fn get_multiple_balances(&self, accounts: &[Address]) -> Result<Vec<U256>, Vec<u8>> {
        // 0.8.3에서는 메모리 캐시를 활용합니다
        let mut balances = Vec::with_capacity(accounts.len());
        for account in accounts {
            balances.push(self.balances.get(*account));
        }
        Ok(balances)
    }
} 