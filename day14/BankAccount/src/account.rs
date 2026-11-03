#[derive(Debug)]
pub struct Account {
    pub user_name: String,
    pub balance: f64,
}

impl Account {
    pub fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: f64) {
        if self.balance >= amount {
            println!("Withdrawing ${} from account.", amount);
            self.balance -= amount;
        } else {
            println!("Insufficient funds for withdrawal.");
        }
    }
    pub fn display(&self) -> f64 {
        self.balance
    }
}
