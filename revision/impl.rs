struct Rectangle{
    width: u32,
    height: u32,
}


impl Rectangle{
    // associated function
    // this doen't take self as parameter and can be called using the struct name like Rectangle::create_new(30, 50)
    fn create_new(width: u32, height: u32) -> Self{
        Self{
            width,
            height,
        }
    }
    
    // methods
    // this takes &self and &mut self as parameter and can be called using the instance of the struct like rect1.area() or rect1.perimeter()
    fn area(&self) -> u32{
        self.width * self.height
    }
    fn perimeter(&self) -> u32{
        2 * (self.width + self.height)
    }
    fn resize(&mut self, new_width: u32, new_height: u32){
        self.width = new_width;
        self.height = new_height;
    }
    // when we put self as parameter in the method it will take ownership of the instance of the struct and we can no longer use that instance after calling this method and if we want we can use it by returning it back to the caller like this 
    fn del(self){
        println!("The rectangle with width {} and height {} is deleted.", self.width, self.height);
    }
}

fn main(){
    let rect1 = Rectangle::create_new(30, 50);

    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("The perimeter of the rectangle is {} pixels.", rect1.perimeter());
    rect1.del();
    println!("The rectangle is deleted.");
    // this will throw error because the ownership of rect1 is moved to the del method and we can no longer use it after that
   // println!("The area of the rectangle is {} square pixels.", rect1.area()); 
}