/*
 * ex04_bonus_bank.rs — Bonus Exercise
 *
 * Task: Define a BankAccount struct with owner (String) and balance (f64).
 *       Methods: deposit(amount), withdraw(amount) -> Result<f64, String>,
 *                balance() -> f64.
 *       Withdraw should return Err if insufficient funds.
 *
 * Run: ./ex04_bonus_bank
 * Expected: Deposit works, withdraw works, insufficient funds returns error
 */

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn new(owner: &str) -> BankAccount {
        BankAccount {
            owner: String::from(owner),
            balance: 0.0,
        }
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) -> Result<f64, String> {
        if amount > self.balance {
            Err(format!(
                "Insufficient funds: balance={:.2}, requested={:.2}",
                self.balance, amount
            ))
        } else {
            self.balance -= amount;
            Ok(self.balance)
        }
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    let mut account = BankAccount::new("Alice");

    // Initial balance
    println!("{}'s balance: ${:.2}", account.owner, account.balance());
    assert!((account.balance() - 0.0).abs() < 0.001);

    // Deposit
    account.deposit(500.0);
    println!("After deposit: ${:.2}", account.balance());
    assert!((account.balance() - 500.0).abs() < 0.001);

    // Successful withdraw
    match account.withdraw(100.0) {
        Ok(new_balance) => println!("Withdrew $100. New balance: ${:.2}", new_balance),
        Err(e) => println!("Withdraw failed: {}", e),
    }
    assert!((account.balance() - 400.0).abs() < 0.001);

    // Insufficient funds
    match account.withdraw(999.0) {
        Ok(_) => println!("Should not succeed"),
        Err(e) => println!("Expected error: {}", e),
    }
    // Balance unchanged
    assert!((account.balance() - 400.0).abs() < 0.001);

    // Exact balance withdraw
    match account.withdraw(400.0) {
        Ok(new_balance) => println!("Withdrew $400. New balance: ${:.2}", new_balance),
        Err(e) => println!("Withdraw failed: {}", e),
    }
    assert!((account.balance() - 0.0).abs() < 0.001);

    println!("All tests passed!");
}
