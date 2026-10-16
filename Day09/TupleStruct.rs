// Tuple Struct 
// by default struct doen't have copy and clone trait, so we need to implement it manually
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit struct 
struct AlwaysEqual;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let subject = AlwaysEqual;
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("hdhs@gmail.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        active: true,
        ..user1};

    println!("User2 email: {}", user2.email);
   // println!("User email: {}", user1.email); // this will throw error because user1 is moved to user2, so we can't use it anymore
   println!("user active: {}", user1.active);// this will work because active is a bool, which implements the Copy trait, so it can be copied to user2 without moving ownership
   println!("user sign_in_count: {}", user1.sign_in_count);// this will work because sign_in_count is a u64, which implements the Copy trait

   let black = Color(0, 0, 0);
   let origin = Point(0, 0, 0);
   let balck2 = black; // this will move the ownership of black to balck2, so we can't use black anymore

    }