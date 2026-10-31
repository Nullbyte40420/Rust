enum IPAddr {
    // v4 is variant of the enum IPAddr, and it can hold 4 u8 values
    // we can put data inside the enum variant like this v4(u8, u8, u8, u8) 
    // after this we can create an instance of the enum like this let ip1 = IPAddr::V4(127, 0, 0, 1);
    V4(u8, u8, u8, u8),
    V6(String),
}

#[allow(dead_code)]// this will avoid the warning of unused fields in the instance of the struct
enum Option<T> {
    Some(T),
    None,
}

impl IPAddr {
    fn display(&self) {
        match self {
            IPAddr::V4(a, b, c, d) => println!("IPv4 Address: {}.{}.{}.{}", a, b, c, d),
            IPAddr::V6(addr) => println!("IPv6 Address: {}", addr),
        }
    }
    fn change_ip(&mut self, new_ip: IPAddr) {
        *self = new_ip;
    }
    
    // this is more efficient way to change the ip address of ip1 without creating a new instance of the enum
    fn change_ip_v4(&mut self, a: u8, b: u8, c: u8, d: u8) {
        *self = IPAddr::V4(a, b, c, d);
    }

    fn delete(self) {
        match self {
            IPAddr::V4(a, b, c, d) => println!("IPv4 Address {}.{}.{}.{} is deleted.", a, b, c, d),
            IPAddr::V6(addr) => println!("IPv6 Address {} is deleted.", addr),
        }
    }
}

#[allow(unused_mut)]
fn main() {
    let mut ip1 = IPAddr::V4(127, 0, 0, 1);
    let mut ip2 = IPAddr::V6(String::from("::1"));


    ip1.display();
    ip2.display();
    ip1.change_ip(IPAddr::V4(192, 168, 1, 1));
    ip1.display();
    ip1.change_ip_v4(10, 0, 0, 1); // more efficient way to change the ip address of ip1 without creating a new instance of the enum
    ip1.display();
    ip1.delete();
    ip2.delete();

    let some_number: Option<i32> = Option::Some(5);
    let some_string: Option<String> = Option::Some(String::from("Hello"));
    match some_number {
        Option::Some(value) => println!("The number is: {}", value),
        Option::None => println!("No number"),
    }
    match some_string {
        Option::Some(value) => println!("The string is: {}", value),
        Option::None => println!("No string"),
    }

    let none_number: Option<i32> = Option::None;
    match none_number {
        Option::Some(value) => println!("The number is: {}", value),
        Option::None => println!("No number"),
    }
}
