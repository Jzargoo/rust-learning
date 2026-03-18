pub struct BankAccount {
    owner: String,
    balance: i32
}

impl BankAccount {
    pub fn new(owner: &str) -> Self {
        Self {
            owner: owner.to_string(),
            balance: 0
        }
    }
    
    pub fn deposit(&mut self, amount: i32) {
        self.balance += amount;
    }

    pub fn widthdraw(&mut self, amount: i32) -> bool {
        
        if self.balance >= amount {
            
            self.balance -= amount;
            true
        
        } else {
            false
        }
    }

    pub fn balance(&self) -> i32 {
        self.balance
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }
}