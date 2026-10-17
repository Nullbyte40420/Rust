// struct 

struct Rectangle {
    width: u32,
    height: u32,
}

// method this applies to the struct Rectangle
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // method that takes another Rectangle as a parameter and returns a boolean indicating whether the current rectangle can hold the other rectangle
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height

    }
    // associated function that creates a new Rectangle instance with the given width and height
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", rect1.area());
    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    let rect3 = Rectangle::new(10, 20);
    println!("The area of the rectangle is {} square pixels.", rect3.area());
}