


// REFERENCING .........

fn main() {
    let mut _x: i32 = 5;
    let _r: &mut i32 = &mut _x; // Refenced _x

    *_r += 1;
    *_r -= 8;

    println!("Value of _x: {}", _x);
    // println!("Value of _r: {}", _r);
}

STRUCT: A data structure that allows you to group multiple fields together under one name.

fn main() {
    let mut account: BankAccount = BankAccount {
        owner: "Kayrose".to_string(),
        balance: 1200.55,
    };

    // Immutable borrow to check the balance.
    account.balance_check();

    // Mutable borrow to withdraw money.
    account.withdraw(300.05);
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
        self.balance_check();
    }

    fn balance_check(&self) {
        println!("account owned by {} has a balance of {}", self.owner, self.balance);
        // self.balance;
    }
}



