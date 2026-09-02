fn main(){
    // rust has three types of loops: loop, while and for

    // loop
    // loop {
    //     println!("This is an infinite loop");
    //     // this loop will  continue till we don't stop it explicitly using ctrl + c or by using break statement
    //     break ;
    // }

    // returning a value from a loop
    let mut x = 0;
    let y  = loop {
        x += 1;
        if x == 10 {
            break x * 2; // this will return the value of x * 2 when the loop is broken
        }
    };
    println!("The value of y is: {y}"); // this will print 20
}