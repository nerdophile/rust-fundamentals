#[derive(Debug)]
struct Account {
    balance: i32,
    id: u32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }

    fn deposit(&mut self, amount: i32) -> i32 {
        self.balance += amount;

        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32 {
        self.balance -= amount;
        self.balance
    }

    fn summary(&self) -> String {
        format!("Account ID: {}, Holder: {}, Balance: {}", self.id, self.holder, self.balance)
    }
}

fn print_account(account: Account) {
    println!(
        "Account ID: {}, Holder: {}, Balance: {}",
        account.id,
        account.holder,
        account.balance
    );
}

#[derive(Debug)]
struct Bank {
    name: String,
    accounts: Vec<Account>,
}

impl Bank {
    fn new(name: String) -> Self {
        Bank {
            name,
            accounts: vec![],
        }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn total_balance(&self) -> i32 {
        self.accounts
            .iter()
            .map(|account| account.balance)
            .sum()
    }

    fn summary(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|account| account.summary())
            .collect::<Vec<String>>()
    }
}

fn main() {
    let mut bank = Bank::new("AMEX".to_string());
    let mut account = Account::new(1, String::from("venky"));

    account.deposit(1000);
    account.withdraw(50);
    bank.add_account(account);

    println!("{}", bank.total_balance());
    println!("{:#?}", bank.summary());
}
