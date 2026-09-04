fn main(){
    let mut s:&str  = "hello";
    s =  "worldd"; // this s is now refers to the new string "world"

    // mutable means that the variable can be changed to refer to a different value, but it does not mean that the value itself can be changed. In this case, s is a mutable reference to a string slice, so we can change what it points to, but we cannot change the contents of the string slice itself.
    let mut s1 = String::from("hello"); // this is a mutable String, which means we can change its contents.
    s1.push_str(" world"); // this will change the contents of s1 to "hello world"
    println!("{}", s1); // this will print "hello world"
}