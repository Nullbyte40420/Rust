fn fibbo(n:i32)  {
    if n <= 0 {
        return;
    } else if n == 1 {
        println!("0");
    } else if n == 2 {
        println!("0, 1");
    } else {
        let mut a = 0;
        let mut b = 1;
        print!("0, 1");
        for _ in 2..n {
            let c = a + b;
            print!(", {c}");
            a = b;
            b = c;
        }
        println!();
    }
    
}
fn fibborec(n:i32) -> i32 {
    if n <= 1{
        return n;
    }

    return fibborec(n-1) + fibborec(n-2);
}
fn main() {
    let n = 10;
    println!("Fibonacci series up to {n} terms:");
    fibbo(n);
    for i in 0..n{
        print!("{} ", fibborec(i));
    }
    println!();
}