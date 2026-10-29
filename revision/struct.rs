
#[allow(dead_code)] // this will avoid the warning of unused fields in the instance of the struct 
#[derive(Clone)] // this will implement the Copy and Clone traits for the struct
// #[derive(cope, Clone)] // this will implement the Copy and Clone traits for the struct
// but we can't implement Copy trait for struct because it has String type fields which doesn't implement Copy trait
struct User{
    user_name: String,
    email: String,
    age: u32,
    signed_in: bool,
}

#[allow(unused_mut)] // this will avoid the warning of unused mutable variable 
fn main() {
    let mut user1 = User {
        user_name: String::from("JohnDoe"),
        email: String::from("hahshush@gmail.com"),
        age: 30,
        signed_in: true,
    };

    println!("User Name: {}", user1.user_name);

    let mut user2 = User{
        email : String::from("hhhhh@gmail.com"),
        user_name : String::from("JeDoe"),
        ..user1 
    };
    // in this we have already asign new values to the email and user_name fields of user2 and we are using the ..user1 syntax to copy the remaining fields from user1 to user2
    println!("User Name: {}", user2.user_name);
    println!("User2 Email: {}, Age: {}", user2.email, user2.age);
    println!("User1 Email: {}, Age: {}", user1.email, user1.age);
    
    // 
    let user3 = User{
        age : 25,
        ..user2
    };
    println!("User3 Email: {}, Age: {}", user3.email, user3.age);
   // println!("User2 Email: {}, Age: {}", user2.email, user2.age);
   // this will throw an error because of partial move of user2 to user3
   // when we copy the user2 to user3 the values of those field who doesn't have 
   // Copy trait will be moved to user3 and user2 will no longer have access to those fields
   // for example the email field of user2 will be moved to user3 and user2 will no longer have access to it
   // but the age field of user2 will be copied to user3 because it has Copy trait and user2 will still have access to it

   user2.email = String::from("newmail@gmail.com");
   user2.user_name = String::from("newname");
   println!("User2 Email: {}, Age: {}", user2.email, user2.age);


   // full move of user2 to user4 
   // let user4 = user2;
   // println!("User4 Email: {}, Age: {}", user4.email, user4.age);
   //println!("User2 Email: {}, Age: {}", user2.email, user2.age);// this will throw an error because of full move of user2 to user4
   // this happen because by default struct doesn't have copy trait 
   // to implement copy trait for struct we can use #[derive(Copy, Clone)] above the struct definition

   let user4 = user2.clone();// this will create a copy of user2 and assign it to user4
   println!("User4 Email: {}, Age: {}", user4.email, user4.age);
   println!("User2 Email: {}, Age: {}", user2.email, user2.age);

}