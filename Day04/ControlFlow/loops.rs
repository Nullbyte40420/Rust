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

    let mut counter = 0;
    `counter: loop {
        println!("Counter: {counter}");
        let mut remaining = 10;

        loop {
            println!("Remaining: {remaining}");
            if remaining == 9 {
                break; // this will break the inner loop
            }
            if counter == 2 {
                break 'counter; // this will break the outer loop
            }
            remaining -= 1;
        }
        counter += 1;
    }
    println!("Counter: {counter}"); // this will print 2
}