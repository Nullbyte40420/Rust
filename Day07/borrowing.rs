fn main(){
    let mut s = String::from("hello");
    let s1 = &mut s;// mutable borrow of s
    s1.push_str(", world!");
    println!("{}", s1);
    let s2 = &mut s;// mutable borrow of s
    s2 .push_str(" How are you?");
    println!("{}", s2);
   // println!("{}", s1); // This line would cause a compile-time error because s1 is no longer valid after s2 is created.
}