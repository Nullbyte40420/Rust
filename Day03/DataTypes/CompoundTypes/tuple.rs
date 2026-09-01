fn main(){
    // tuple is a compound type which can hold multiple values of different types
    // tuple can't be resize at any cost at all it can not shrink or grow 
    //     : (i32, f64, u8) is optional because Rust can infer the type of the tuple from the values assigned to it
    let tup: (i32, f64, u8) = (500, 6.4, 1); // tuple is a compound type which can hold multiple values of different types
    //different ways to define a tuple
    let tup1 = (500, 6.4, 1); 
    const TUPLE_NAME_ONE: (i32, f64, u8) = (500, 6.4, 1); 
    let mut tup2 = (500, 6.4, 1);
    // mutability of a tuple is determined by the mutability of the variable it is assigned to, not the tuple itself
    // let tup3 = (500, 6.4, 1); // this is an immutable tuple because it is assigned to an immutable variable this follow basic things of normal let key word behaviours like shadowing 
    // let mut tup4 = (500, 6.4, 1); // this is a mutable tuple because it is assigned to a mutable variable this follow basic things of normal let mut key word behaviours like shadowing and mutability
    tup2.0 = 600; // this is possible because tup2 is a mutable variable and it can be reassigned to a new value
    //tup2.0 = "hello"; // this is not possible because tup2 is a tuple of type (i32, f64, u8) and it cannot be reassigned to a new value of a different type
    // const TUPLE_NAME_TWO: (i32, f64, u8) = (500, 6.4, 1); // this is a constant tuple because it is assigned to a constant variable this follow basic things of normal const key word behaviours like shadowing and mutability

    // to get the values of a tuple we can use destructuring or indexing

    // pattern matching or destructuring
    let tup3 = (500, 6.4, 1);
    let (x, y, z) = tup3; // this is destructuring
    println!("The value of x is: {}", x); // this will print 500

    // indexing or period notation
    let tup4 = (500, 6.4, 1);
    let first_value = tup4.0; // this is indexing
    println!("The value of first_value is: {}", first_value); // this will print 500
    println!("The value of y is: {}", tup4.1); // this will print 6.4


    // unit 
    let unit = (); // unit is a compound type which can hold no value and it is represented by an empty tuple
    println!("The value of unit is: {:?}", unit); // this will print ()

}