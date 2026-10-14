// this is a struct
struct user{
    name: String,
    active:bool,
    email:String,
    sign_in_count:u64,
}

fn main(){

    // this is a instance of struct
    let mut user1 = user{
        //We don’t have to specify the fields in the same order in which we declared them in the struct
        name: String::from("John"),
        active: true,
        email: String::from("john@example.com"),
        sign_in_count: 1,
    };

    // we can access the fields of a struct using dot notation
    println!("Name: {}", user1.name);
    //If the instance is mutable, we can change a value by using the dot notation and assigning into a particular field
    user1.name = String::from("Jane");
    println!("Updated Name: {}", user1.name);

    // Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable


    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // short cut 
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}



fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

//Using the Field Init Shorthand
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

