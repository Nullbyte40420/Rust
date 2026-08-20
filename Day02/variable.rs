//let x = 10; // variable can not be used in global scope
const HHH: i32 = 10; // constant can be used in global scope

fn main(){
    let x = 10; // x is an variable which is immutable by default and pointing to an integer value 10
    //x = 20; // this will throw an error because x is immutable E0384 (immutable variable was reassigned)
    let mut y = 10; // y is a mutable variable which is pointing to an integer value 10 
    y = 20; // this will not throw an error because y is mutable and can be reassigned to a new value
    const CONSTANT_NAME_ONE: i32 = 10; // CONSTANT_NAME_ONE is a constant variable which is pointing to an integer value 10
    // constant cannot be mutable like a variable and it cannot be reassigned to a new value

    // shadowing 
    let mut z = 10;
    let z = z + 10; // this is shadowing, the new variable z is shadowing the previous variable z
    {
        let z = z + 10; // this is shadowing, the new variable z is shadowing the previous variable z
        println!("The value of z in the inner scope is: {}", z); // this will print 30
        // shadowing can be end when scope is ended or when the variable is reassigned to a new value
    }
    println!("The value of z in the outer scope is: {}", z); // this will print 20
    const CONSTANT_NAME_TWO: i32 = 10;
    //const CONSTANT_NAME_TWO: i32 = 20; // this is not possible because constant cannot be reassigned to a new value
    {
        const CONSTANT_NAME_TWO: i32 = 30; // this is possible because constant can be shadowed in a new scope
    }
    let mut a = 10; // a is a mutable variable which is pointing to an integer value 10
    //x = "string"; // this is wrong in Rust because x is an integer variable and cannot be reassigned to a string value
    let a = "string"; // this is possible because x is a new variable which is shadowing the previous variable x and it is pointing to a string value

 }