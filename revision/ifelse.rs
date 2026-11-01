#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

// let...else ka use karke clean aur early-return wala function
fn describe_state_quarter(coin: Coin) -> Option<String> {
    // Agar coin Quarter nahi hai, toh yahin se turant None return ho jayega
    // Aur agar Quarter hai, toh 'state' variable extract ho kar aage mil jayega!
    let Coin::Quarter(state) = coin else {
        return None;
    };

    // Yeh hai hamara "Happy Path" (bina kisi nested else ke)
    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn main() {
    let quarter_alaska = Coin::Quarter(UsState::Alaska);
    let quarter_alabama = Coin::Quarter(UsState::Alabama);
    let penny = Coin::Penny;

    // Test cases
    if let Some(desc) = describe_state_quarter(quarter_alaska) {
        println!("{}", desc);
    }

    if let Some(desc) = describe_state_quarter(quarter_alabama) {
        println!("{}", desc);
    }

    // Yeh None return karega kyunki yeh Quarter nahi hai
    match describe_state_quarter(penny) {
        Some(desc) => println!("{}", desc),
        None => println!("Not a state quarter, so no description available!"),
    }
}