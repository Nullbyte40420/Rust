fn main() {
    let mut s = String::from("Rust");

    let r1 = &s;
    println!("r1: {}", r1);

    let mut s = s; 
    s.push_str(" Lang");

    let r2 = &s;
    println!("r2: {}", r2);
}