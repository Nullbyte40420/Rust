// predicts the output 


fn main() {

    // code 1 
    {
    let mut s1 = String::from("hello");
    
    let r1 = &s1; 
    let r2 = &s1; 
    println!("{} and {}", r1, r2);

    let r3 = &mut s1;
    println!("{}", r3);

    //this will run successfully because the immutable references r1 and r2 are no longer used after their println! statement, allowing the mutable reference r3 to be created without any conflicts.
    }
    //code 2
    {
    let s1 = String::from("hello");
    let s2 = s1;

     //  println!("{}", s1);
    }

}