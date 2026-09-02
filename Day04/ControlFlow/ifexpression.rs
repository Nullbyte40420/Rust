fn main(){
    let mut x:i32 = 3;
    if x > 5 { // if block is a conditional statement which is used to execute a block of code if the condition is true
        // condition must be a boolean expression which evaluates to true or false
        println!("x is greater than 5");
    } else { // else block is optional 
        println!("x is not greater than 5");
    }

    // if else if 
    let x = 8 ;
    if x > 5 {
        println!("x is greater than 5");
    } else if x == 5 {
        println!("x is equal to 5");
    } else {
        println!("x is less than 5");
    }

    // using if in statements as an expression to assign a value to a variable
    let y = if x > 5 { 10 // this is the value that will be assigned to y if the condition is true
    } else { 5 // this is the value that will be assigned to y if the condition is false 
        };

    let z = if x > 5 { 10 } else { 5 }; // this is a more concise way to write the same thing as above

    // but the values in both arm must be same type otherwise it will throw an error
    // let a = if x > 5 { 10 } else { "hello" }; // this will throw an error because the values in both arm are not of same type
}