// 1. Enums define kiye gaye hain (Coin aur UsState)
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // Variant ke andar data store hai
}

// 2. Coin ki value nikalne ke liye match ka use (Data binding ke sath)
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!"); // Multi-line code ke liye curly brackets
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // 'state' variable yahan data bind (extract) kar raha hai
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// 3. Option<T> ke sath match ka use (plus_one function)
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,             // Agar value nahi hai
        Some(i) => Some(i + 1),   // Agar value hai, toh 'i' mein extract karke 1 jod do
    }
}

fn main() {
    // Coins par match chalana
    let coin1 = Coin::Penny;
    let coin2 = Coin::Quarter(UsState::Alaska);

    println!("Value: {} cents", value_in_cents(coin1));
    println!("Value: {} cents", value_in_cents(coin2));

    // Option<T> par match chalana
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    match six {
        Some(val) => println!("Result after plus_one: {}", val),
        None => println!("Got None"),
    }

    match none {
        Some(val) => println!("Result: {}", val),
        None => println!("Got None as expected!"),
    }

    // 4. Catch-all patterns aur _ placeholder ka use (Dice roll example)
    let dice_roll = 9;
    match dice_roll {
        3 => println!("Player got a fancy hat!"),
        7 => println!("Player lost a hat!"),
        other => println!("Player moved {} spaces.", other), // Named catch-all variable
    }

    // _ placeholder ka use jab value ki zaroorat na ho
    let another_roll = 5;
    match another_roll {
        3 => println!("Hat added"),
        7 => println!("Hat removed"),
        _ => println!("Reroll the dice!"), // Underscore ignore karega baki sabhi values ko
    }

    // Unit value () ka use jab kuch bhi na karwana ho
    let final_roll = 2;
    match final_roll {
        3 => println!("Hat added"),
        7 => println!("Hat removed"),
        _ => (), // Kuch nahi karega baki values par
    }
}