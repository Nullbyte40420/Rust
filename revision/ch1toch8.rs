// =====================================================================
// RUST REVISION: CHAPTER 1 TO CHAPTER 8 (one file, run it with cargo run)
// How to use: read a section, close the file, try to write it yourself, then compare.
// Lines starting with "// ERROR:" are code that does NOT compile. Uncomment ONE at a time,
// read the compiler message, then comment it again.
// =====================================================================

use std::collections::HashMap; // HashMap is not in the prelude, so we bring it in (ch8)

// ---------------------------------------------------------------------
// CH7: MODULES AND PRIVACY (inline modules, same rules as separate files)
// ---------------------------------------------------------------------
mod bank {
    // pub on the struct does NOT make its fields pub, each field needs its own pub
    pub struct Account {
        pub owner: String,
        balance: u32, // private: only code inside `bank` (and its child modules) can touch it
    }

    impl Account {
        // associated function (no self), called as Account::new(...)
        pub fn new(owner: &str) -> Account {
            Account { owner: owner.to_string(), balance: 0 }
        }

        // method with &mut self, because it changes the account
        pub fn deposit(&mut self, amount: u32) {
            self.balance += amount;
        }

        // method with &self, because it only reads
        pub fn balance(&self) -> u32 {
            self.balance
        }
    }

    // child module: a child CAN see private items of its parent (super = the parent module)
    pub mod audit {
        pub fn peek(a: &super::Account) -> u32 {
            a.balance // works, audit is inside bank
        }
    }
}

// use brings a path into scope so we don't write bank::Account every time
use bank::Account;

// ---------------------------------------------------------------------
// CH5: STRUCTS
// ---------------------------------------------------------------------
#[derive(Debug)] // lets us print with {:?} and {:#?}
struct User {
    name: String,
    email: String,
    active: bool,
}

#[derive(Debug)]
struct Rect {
    w: u32,
    h: u32,
}

impl Rect {
    // associated function: no self, called with Rect::square(4), NOT with r.square(4)
    fn square(size: u32) -> Self {
        Self { w: size, h: size }
    }

    // &self: borrow to read
    fn area(&self) -> u32 {
        self.w * self.h
    }

    fn can_hold(&self, other: &Rect) -> bool {
        self.w > other.w && self.h > other.h
    }

    // &mut self: borrow to change
    fn grow(&mut self, by: u32) {
        self.w += by;
        self.h += by;
    }

    // self (by value): takes ownership, the caller can't use the Rect after this call
    fn consume(self) -> u32 {
        self.w * self.h
    }
}

struct Color(i32, i32, i32); // tuple struct: fields have no names, access with .0 .1 .2

struct Person {
    name: String,
    age: u32,
}

// field init shorthand: if the variable name equals the field name we write it once
fn build_user(name: String, email: String) -> User {
    User { name, email, active: true }
}

// ---------------------------------------------------------------------
// CH6: ENUMS AND MATCH
// ---------------------------------------------------------------------
#[derive(Debug)]
enum Shape {
    Circle(f64),                 // variant holding one value
    Square(f64),
    Rectangle { w: f64, h: f64 }, // variant holding named fields
    Empty,                       // variant holding nothing
}

impl Shape {
    fn area(&self) -> f64 {
        // match must handle EVERY variant (exhaustive), or it does not compile
        match self {
            Shape::Circle(r) => 3.14 * r * r,
            Shape::Square(s) => s * s,
            Shape::Rectangle { w, h } => w * h,
            Shape::Empty => 0.0,
        }
    }
}

#[derive(Debug)]
enum Msg {
    Quit,
    Write(String),
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// ---------------------------------------------------------------------
// CH4: functions that show ownership and borrowing
// ---------------------------------------------------------------------
fn takes_ownership(s: String) {
    println!("takes_ownership got: {s}");
} // s is dropped here, the caller's String is gone

fn calculate_length(s: &String) -> usize {
    s.len() // we only borrowed s, so nothing is dropped
}

fn change(s: &mut String) {
    s.push_str(", world"); // needs &mut String, and the caller needs a `mut` variable
}

// &str (not &String) is better: it accepts both String and string literals
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// ERROR: dangling reference. s dies at the end of the function, but we return a reference to it
// fn dangle() -> &String {
//     let s = String::from("hi");
//     &s
// }

// ---------------------------------------------------------------------
// CH3: functions need types on parameters and return value
// ---------------------------------------------------------------------
fn add_one(x: i32) -> i32 {
    x + 1 // no semicolon: this is the return value (an expression)
    // x + 1;  <- with semicolon it becomes a statement, the function returns () and won't compile
}

fn main() {
    println!("=== CH1: hello cargo ===");
    // cargo new name  -> make project, cargo run -> build and run, cargo check -> only check errors
    // cargo build --release -> optimized build. Comments start with //. println! is a macro (the !).
    println!("Hello, world!");

    ch3_basics();
    ch4_ownership();
    ch5_structs();
    ch6_enums();
    ch7_modules();
    ch8_collections();
    weak_spots();
}

fn ch3_basics() {
    println!("\n=== CH3: variables, types, control flow ===");

    // variables are immutable by default, mut makes them changeable
    let x = 5;
    // ERROR: x = 6; // cannot assign twice to immutable variable
    let mut y = 10;
    y += 1;
    println!("x = {x}, y = {y}");

    // const: always immutable, needs a type, can be declared anywhere
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS = {MAX_POINTS}");

    // shadowing: a NEW variable with the same name (can even change the type)
    let x = x + 1; // x is 6 now
    {
        let x = x * 2; // inner x is 12, only inside this block
        println!("inner x = {x}");
    }
    println!("outer x = {x}"); // 6 again
    let spaces = "   ";
    let spaces = spaces.len(); // &str -> usize is fine with shadowing
    println!("spaces = {spaces}");
    // ERROR: let mut name = "abc"; name = name.len(); // mut can change the value, NOT the type

    // scalar types
    let a: i32 = -42; // signed ints: i8 i16 i32 i64 i128 isize
    let b: u8 = 255; // unsigned ints: u8 u16 u32 u64 u128 usize (255 is the max of u8)
    let c: f64 = 3.14; // floats: f32 f64 (f64 is default)
    let d: bool = true;
    let e: char = 'z'; // char uses single quotes and is 4 bytes
    println!("{a} {b} {c} {d} {e}");

    // compound types: tuple (mixed types, fixed size) and array (same type, fixed size)
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (t1, t2, t3) = tup; // destructuring
    println!("{} {} {} | {} {} {}", tup.0, tup.1, tup.2, t1, t2, t3);
    let arr = [1, 2, 3, 4, 5];
    let arr2 = [3; 5]; // [3, 3, 3, 3, 3]
    println!("{} {:?}", arr[0], arr2);
    // arr[10] would compile if the index is a variable, but PANIC at runtime (index out of bounds)

    // functions
    println!("add_one(5) = {}", add_one(5));

    // if is an expression, both branches must have the SAME type
    let size = if y > 5 { "big" } else { "small" };
    println!("size = {size}");
    // ERROR: let n = if true { 5 } else { "six" }; // incompatible types

    // loops: loop (infinite, can return a value with break)
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 5 {
            break counter * 2; // break with a value
        }
    };
    println!("loop result = {result}");

    // loop labels: plain `break` stops only the INNERMOST loop, `break 'label` stops the labeled one
    let mut count = 0;
    'outer: loop {
        let mut remaining = 3;
        loop {
            if remaining == 1 {
                break; // breaks only the inner loop
            }
            if count == 2 {
                break 'outer; // breaks the outer loop
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("count = {count}"); // 2

    // while and for
    let mut n = 3;
    while n != 0 {
        println!("while: {n}");
        n -= 1;
    }
    for i in (1..4).rev() { // 1..4 is 1,2,3 (end not included), .rev() reverses
        println!("for rev: {i}");
    }
    for item in &arr { // iterating by reference
        print!("{item} ");
    }
    println!();
}

fn ch4_ownership() {
    println!("\n=== CH4: ownership, borrowing, slices ===");

    // RULES: 1) every value has one owner 2) only one owner at a time 3) owner goes out of scope, value is dropped

    // move: String lives on the heap, assigning moves ownership
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved into s2, s1 is invalid now
    // ERROR: println!("{s1}"); // borrow of moved value
    let s3 = s2.clone(); // clone makes a real deep copy
    println!("{s2} {s3}");

    // Copy types (ints, bool, char, f64, tuples of Copy) are copied, not moved
    let n = 5;
    let m = n;
    println!("{n} {m}");

    // passing to a function moves it too
    takes_ownership(s3);
    // ERROR: println!("{s3}"); // s3 was moved into the function

    // borrowing with &: we can read, but we don't own
    let s4 = String::from("yo");
    let len = calculate_length(&s4);
    println!("len of {s4} is {len}"); // s4 still usable

    // &mut: we can change, and the variable must be declared mut
    let mut s5 = String::from("hello");
    change(&mut s5);
    println!("{s5}");

    // BORROW RULES: any number of & readers OR exactly one &mut writer, never both at the same time
    // a reference lives from where it is created till its LAST USE (not till the end of the block)
    let r1 = &s5;
    let r2 = &s5;
    println!("{r1} {r2}"); // last use of r1 and r2
    let r3 = &mut s5; // fine, r1 and r2 are not used anymore
    r3.push('!');
    println!("{r3}");
    // ERROR: let a = &mut s5; let b = &mut s5; a.push('x'); b.push('y'); // two &mut at once

    // slices: a reference to a part of a collection
    let sentence = String::from("hello world");
    let word = first_word(&sentence);
    let hello = &sentence[0..5];
    let world = &sentence[6..];
    let whole = &sentence[..];
    println!("{word} | {hello} | {world} | {whole}");
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // [2, 3]
    println!("{:?}", slice);
    // ERROR: let mut s = String::from("hi there"); let w = first_word(&s); s.clear(); println!("{w}");
    // s.clear() needs &mut s while w (an immutable borrow) is still used later
}

fn ch5_structs() {
    println!("\n=== CH5: structs and methods ===");

    let u1 = build_user(String::from("abhi"), String::from("a@x.com"));
    // struct update syntax: take the remaining fields from u1
    // the String field `name` is MOVED out of u1, the bool `active` is copied
    let u2 = User { email: String::from("b@x.com"), ..u1 };
    // ERROR: println!("{}", u1.name); // name was moved
    println!("{} {}", u1.email, u1.active); // these were not moved, so they still work
    println!("{:#?}", u2); // {:#?} is the pretty multi-line debug print

    // methods vs associated functions
    let mut r = Rect::square(4); // associated function, called with ::
    println!("area = {}", r.area()); // method, called with .
    r.grow(2); // &mut self, so r must be `let mut`
    let small = Rect { w: 1, h: 1 };
    println!("{:?} can hold {:?}: {}", r, small, r.can_hold(&small));
    // ERROR: r.square(5); // square has no self, it is NOT a method
    let total = r.consume(); // self by value: r is moved into consume
    // ERROR: println!("{}", r.area()); // r was moved
    println!("total = {total}");

    // tuple struct
    let black = Color(0, 0, 0);
    println!("{} {} {}", black.0, black.1, black.2);

    // partial move: moving one field out of a struct
    let p = Person { name: String::from("raj"), age: 20 };
    let n = p.name; // name is moved out of p
    println!("{n} {}", p.age); // age (Copy) and other untouched fields still usable
    // ERROR: println!("{}", p.name); // the moved field
    // ERROR: show(&p); // can't borrow the whole struct after a partial move
}

fn ch6_enums() {
    println!("\n=== CH6: enums, match, Option ===");

    let shapes = [
        Shape::Circle(2.0),
        Shape::Square(3.0),
        Shape::Rectangle { w: 2.0, h: 5.0 },
        Shape::Empty,
    ];
    for s in &shapes {
        println!("{:?} -> area {}", s, s.area());
    }

    // Option<T> replaces null: Some(value) or None. Rust forces us to handle None
    let a = plus_one(Some(5));
    let b = plus_one(None);
    println!("{:?} {:?}", a, b);
    // ERROR: let x: i32 = 5; let y: Option<i32> = Some(10); let z = x + y; // i32 + Option<i32>
    let y: Option<i32> = Some(10);
    println!("{}", 5 + y.unwrap_or(0)); // get the value out first

    // match with a catch-all
    let number = 3;
    match number {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("something else"), // _ matches everything left
    }

    // if let: a short match when we only care about one pattern
    let max = Some(5u8);
    if let Some(m) = max {
        println!("max is {m}");
    } else {
        println!("no max");
    }

    // match and ownership: matching by value MOVES the data out of the enum
    let m = Msg::Write(String::from("hi"));
    match &m { // &m borrows, so m is NOT moved
        Msg::Write(s) => println!("write: {s}"), // s is &String here
        Msg::Quit => println!("quit"),
    }
    println!("{:?}", m); // works because we matched on &m
    // ERROR: match m { Msg::Write(s) => ..., Msg::Quit => ... } println!("{:?}", m); // m partially moved
}

fn ch7_modules() {
    println!("\n=== CH7: modules and privacy ===");
    let mut a = Account::new("raj");
    a.deposit(50);
    println!("owner = {}, balance = {}", a.owner, a.balance());
    println!("peek = {}", bank::audit::peek(&a)); // the child module can read the private field
    // ERROR: println!("{}", a.balance); // balance is private outside `bank`
    // ERROR: let acc = Account { owner: String::from("x"), balance: 5 }; // private field, can't build it here
    // For files: `mod order;` in main.rs loads src/order.rs, and items inside need `pub` to be used outside
}

fn ch8_collections() {
    println!("\n=== CH8: Vec, String, HashMap ===");

    // ----- Vec<T>: growable list, one type only, stored on the heap -----
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    let v2 = vec![10, 20, 30]; // vec! macro with values
    let third = &v2[2]; // indexing gives &T and PANICS if out of range
    println!("{third}");
    match v2.get(100) { // get gives Option<&T>, no panic
        Some(x) => println!("got {x}"),
        None => println!("no element at 100"),
    }
    println!("{:?}", v);

    let mut v3 = vec![100, 32, 57];
    for i in &mut v3 {
        *i += 50; // * (dereference) reaches the value behind the &mut reference
    }
    for i in &v3 {
        print!("{i} "); // i is &i32 here
    }
    println!();
    // ERROR: let first = &v3[0]; v3.push(6); println!("{first}"); // push may reallocate, first would dangle
    // ERROR: for x in &v3 { v3.push(1); } // the loop holds a borrow of the whole vector

    // enum lets one Vec hold different types
    enum Cell {
        Int(i32),
        Text(String),
    }
    let row = vec![Cell::Int(3), Cell::Text(String::from("blue"))];
    for c in &row {
        match c {
            Cell::Int(n) => println!("int {n}"),
            Cell::Text(s) => println!("text {s}"),
        }
    }

    // ----- String: wrapper over Vec<u8>, always valid UTF-8 -----
    let mut s = String::new();
    s.push_str("foo"); // push_str takes &str, push takes a char
    s.push('!');
    let a = String::from("Hello, ");
    let b = String::from("world");
    let c = a + &b; // a is MOVED (add takes self), b is only borrowed (&String becomes &str)
    // ERROR: println!("{a}"); // a was moved
    let f = format!("{c}-{b}-{s}"); // format! borrows everything and returns a String
    println!("{f}");

    // no s[0] indexing on strings: bytes, chars and letters are different things in UTF-8
    let hindi = String::from("नमस्ते");
    println!("bytes = {}, chars = {}", hindi.len(), hindi.chars().count()); // 18 and 6
    println!("{}", &hindi[0..3]); // works, the first char takes exactly 3 bytes
    // ERROR (panic at runtime): let bad = &hindi[0..2]; // not a char boundary
    for ch in "Зд".chars() {
        print!("{ch} ");
    }
    println!();

    // ----- HashMap<K, V>: lookup by key, order is NOT guaranteed -----
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // same key: the old value is overwritten
    scores.entry(String::from("Yellow")).or_insert(50); // insert only if the key is missing
    scores.entry(String::from("Blue")).or_insert(99); // Blue exists, nothing changes
    let blue = scores.get("Blue").copied().unwrap_or(0); // get gives Option<&i32>
    println!("blue = {blue}, count = {}", scores.len());

    // owned values are MOVED into the map
    let key = String::from("color");
    let mut m2 = HashMap::new();
    m2.insert(key, 1);
    // ERROR: println!("{key}"); // key was moved into the map

    // word counter: the classic entry pattern
    let mut counts = HashMap::new();
    for w in "hello world hello".split_whitespace() {
        let count = counts.entry(w).or_insert(0); // or_insert gives &mut V
        *count += 1; // dereference to change the number
    }
    println!("hello = {:?}, world = {:?}", counts.get("hello"), counts.get("world"));
}

fn weak_spots() {
    println!("\n=== MY WEAK SPOTS (read these again and again) ===");
    // 1. break without a label stops only the INNERMOST loop (ch3)
    // 2. A method with `self` takes ownership, the variable is moved (ch5)
    // 3. match on an enum by value can move data out, use `match &x` to borrow (ch6)
    // 4. Private items are visible to the same module and its CHILD modules, not outside (ch7)
    // 5. A for loop over &v holds a borrow of v, so no push/clear inside the loop (ch8)
    // 6. Associated functions (no self) are called with ::, methods with . (ch5)
    // 7. A reference lives till its LAST USE, so move the println above the conflicting line
    println!("done");
}