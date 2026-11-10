#[derive(Debug, PartialEq)]
pub struct Account {
    owner: String,
    balance: u64,
}

impl Account {
    pub fn new(owner: String) -> Self {
        Self { owner, balance: 0 }
    }

    pub fn owner(&self) -> &str {
        self.owner.as_str()
    }

    pub fn balance(&self) -> u64 {
        self.balance
    }

    pub fn deposit(&mut self, amount: u64) {
        self.balance += amount
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<(), String> {
        if amount > self.balance {
            Err("Insufficient balance".to_string())
        } else {
            self.balance -= amount;
            Ok(())
        }
    }

    pub fn close(self) -> u64 {
        self.balance
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn verify_account_create() {
        let a1 = Account::new("ping".to_string());
        assert_eq!(a1.owner(), "ping".to_string());
        assert_eq!(a1.balance(), 0);
    }

    #[test]
    fn verify_account_owner() {
        let a1 = Account::new("ping".to_string());
        assert_eq!(a1.owner(), "ping".to_string());
    }

    #[test]
    fn verify_account_balance() {
        let a1 = Account::new("ping".to_string());
        assert_eq!(a1.balance(), 0);
    }

    #[test]
    fn verify_account_deposit() {
        let mut a1 = Account::new("ping".to_string());
        assert_eq!(a1.balance(), 0);
        a1.deposit(200);
        assert_eq!(a1.balance(), 200);
    }

    #[test]
    fn verify_account_withdraw() {
        let mut a1 = Account::new("ping".to_string());
        a1.deposit(200);
        assert_eq!(a1.balance(), 200);
        assert!(a1.withdraw(20).is_ok());
        assert_eq!(a1.balance(), 180);
        assert!(a1.withdraw(180).is_ok());
        assert_eq!(a1.balance(), 0);

        assert!(a1.withdraw(181).is_err());
        assert_eq!(a1.balance(), 0);
    }

    #[test]
    fn verify_account_close() {
        let mut a1 = Account::new("ping".to_string());
        a1.deposit(200);
        let closing_bal = a1.close();
        assert_eq!(closing_bal, 200);
        // error[E0382]: borrow of moved value: `a1`
        // a1.deposit(20);
    }
}
