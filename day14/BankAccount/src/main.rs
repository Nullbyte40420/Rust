mod account; 
fn main() {
    let  mut account = account::Account {
        user_name: String::from("John Doe"),
        balance: 1000.0,
    };
    println!("Hello, world!");
    println!("Account: {:#?}", account);
    account.deposit(500.0);
    println!("Account after deposit: {:#?}", account);
    account.withdraw(200.0);
    account.withdraw(2000.0);
    let current_balance = account.display();
    account.withdraw(100.0);
    println!("Current balance: {}", current_balance);
}
