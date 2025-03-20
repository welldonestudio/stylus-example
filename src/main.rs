mod contract;

use crate::contract::{contract, storage}; // contract 모듈 내의 contract와 storage를 가져옵니다.

fn main() {
    // contract 모듈의 Contract 구조체 사용 예시
    let mut contract = contract::Contract::new(Address::from([0u8; 20]));
    contract.increment_balance();
    println!("Contract balance: {}", contract.get_balance());

    // storage 모듈의 StorageExample 구조체 사용 예시
    let mut storage_example = storage::StorageExample {
        some_value: StorageUint::from(0),
        addresses: StorageVec::new(),
    };
    storage_example.add_value(100.into());
    println!("Stored value: {}", storage_example.some_value.get());
}