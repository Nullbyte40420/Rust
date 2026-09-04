fn main() {
    let num:i32 = 10;
    check_even_odd(num);
    count_down(num);
    let sum = add_numbers(5, 7);
    println!("The sum of 5 and 7 is: {}", sum);
}

fn check_even_odd(num:i32){
    if num % 2 == 0 {
        println!("{} is even", num);
    } else {
        println!("{} is odd", num);
    }
}

fn count_down(num:i32){
    for i in (0..=num).rev() {
        if i ==0 {
            println!("Liftoff!");
            break;
        }
        println!("{}", i);
    }
}

fn add_numbers(num1:i32, num2:i32) -> i32 {
    return num1 + num2;
}