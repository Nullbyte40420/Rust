// 1. BASIC ENUM (Jisme sirf names hote hain)
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}


// 2. ADVANCED ENUM (Jiske variants mein data hota hai)

#[derive(Debug)]
enum IpAddr {
    // V4 variant ke andar 4 numbers (u8) hain
    V4(u8, u8, u8, u8),
    // V6 variant ke andar ek String hai
    V6(String),
}


// 3. MESSAGE ENUM (Jisme alag-alag tarah ka data hai)
#[derive(Debug)]
enum Message {
    Quit,                         // Koi data nahi
    Move { x: i32, y: i32 },      // Named fields (struct jaisa)
    Write(String),                // Ek single String
    ChangeColor(i32, i32, i32),   // Teen integers (RGB colors)
}

// 4. IMPL BLOCK (Enum ke upar methods banana)

impl Message {
    // Yeh ek method hai jo 'self' leta hai (yani us instance par chalega)
    fn call(&self) {
        println!("Message ka method call ho gaya! Data: {:?}", self);
    }
}

fn main() {
    // --- PART 1: Basic Enum Usage ---
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("IP Kind: {:?} and {:?}", four, six);

    // --- PART 2: Enum with Direct Data ---
    // Yahan IpAddr::V4 ek constructor ki tarah kaam kar raha hai jo data le raha hai
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("Home IP: {:?}", home);
    println!("Loopback IP: {:?}", loopback);

    // --- PART 3: Message Enum Instances ---
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 10, y: 20 };
    let m3 = Message::Write(String::from("Hello Rust!"));
    let m4 = Message::ChangeColor(255, 0, 128);

    // --- PART 4: Calling Methods on Enum ---
    // Jaise structs par methods chalte hain, waise hi enums par bhi chalte hain!
    m3.call();
}