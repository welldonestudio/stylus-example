
#[storage]
pub struct Contract {
    owner: StorageAddress,
    balance: StorageUint<U256, ()>,
}

impl Contract {
    // Contract 생성자
    pub fn new(owner: Address) -> Self {
        Contract {
            owner: StorageAddress::new(owner),
            balance: StorageUint::new(),
        }
    }

    // 계약의 잔액을 증가시키는 메서드
    pub fn increment_balance(&mut self) {
        let current_balance = self.balance.get();
        self.balance.set(current_balance + 1.into());
    }

    // 잔액을 반환하는 메서드
    pub fn get_balance(&self) -> U256 {
        self.balance.get()
    }
}