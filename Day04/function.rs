fn main(){
    println!("Hello, world!"); 
    another_function();  // calling another function
    let result= add(3,4);// 3 and 4 are concrete values of type i32 which are passed to the function add
    println!("The result is: {}", result);

    let x = 5; // is a statement which is a line of code that performs an action and does not return a value
    // statements ends with a semicolon and do not return a value

    // 5 + 6 // is an expression which is a line of code that evaluates to a value and can be used in a statement
    // function calling is also an expression because it evaluates to a value and can be used in a statement
    // calling an variable is also an expression because it evaluates to a value and can be used in a statement
    let y = {
        let x = 3; 
        x + 1 
    };
    println!("The value of y is: {}", y);
}

//  this can be write anywhere in the file but it is a good practice to write it after the main function
// rust compiler read the file from top to bottom and it is known as function hoisting which means that the function can be called before it is defined in the file
fn another_function() {
    println!("Another function.");
}

// parameters can be passed to functions
fn add(x: i32, y: i32) -> i32 { // this function takes two parameters of type i32 and returns a value of type i32
    x + y // this is the return value of the function
}