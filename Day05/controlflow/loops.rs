fn main() {
    let mut number = 3;

    while number != 0 { // this loop will continue until the condition is false
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a { // this loop will iterate over the elements of the array
        println!("The value is: {element}");
    }

    for number in (1..4+1){ // 4 is exclusive in the range, so we need to add 1 to include it we can also use 1..=4 
        println!("{number}!");
    }

    for number in (1..=4){ // this loop will iterate over the range of numbers from 1 to 4
        println!("{number}!");
    }

    // we can also use the rev() method to reverse the range
    for number in (1..=4).rev(){ // this loop will iterate over the range of numbers from 4 to 1
        println!("{number}!");
    }
}