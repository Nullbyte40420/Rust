fn main() {
    let mut s = String::from("rust");

    let r1 = &s;
    let r2 = &s;

    let r3 = &mut s;

    println!("{}, {}, and {}", r1, r2, r3);
}