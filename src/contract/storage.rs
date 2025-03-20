#[storage]
pub struct StorageExample {
    pub some_value: StorageUint,
    pub addresses: StorageVec<StorageAddress>,
}

impl StorageExample {
    // 새로운 값을 추가하는 메서드
    pub fn add_value(&mut self, value: U256) {
        self.some_value.set(value);
    }

    // 주소를 추가하는 메서드
    pub fn add_address(&mut self, address: Address) {
        self.addresses.push(address);
    }
}
