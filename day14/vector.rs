fn main(){
    // first collection type is a Vec<T> or known as vector, which is a resizable array

    // vector can only hold one type of data, so we need to specify the type of data it will hold
    
    // this will create a empty vector of type i32, which is a signed 32-bit integer
    // Note that we added a type annotation here. Because we aren’t inserting any values into this vector, Rust doesn’t know what type of elements we want to store.
    let mut v: Vec<i32> = Vec::new(); // this will create a empty vector of type i32, which is a signed 32-bit integer

    // vectore are implemented using generics, so we can create a vector of any type, like this:
    let mut v2: Vec<String> = Vec::new(); // this will create a empty vector of type String

    // Rust conveniently provides the vec! macro to create a new vector that contains initial values, like this:
    let v3 = vec![1, 2, 3]; // this will create a vector of type i32

    // updating a vector is done using the push method, which adds an element to the end of the vector, like this:
    v2.push(String::from("Hello")); // this will add "Hello" to the end of the vector v2
    v.push(1); // this will add 1 to the end of the vector v

    // accessing elements in a vector is done using the two ways, using indexing or using the get method, like this:
    let v4 = vec![1, 2, 3, 4, 5]; 
    // indexing is done using the square brackets, like this:
    let third : &i32 = &v4[2]; // this will get the third element of the vector v4, which is 3
    println!("The third element is {third}"); // this will print "The third element is 3"

    // using the get method is done using the get method, like this:
    // the get method returns an Option<&T>, which is an enum that can be either Some(&T) or None
    let third2 : Option<&i32> = v4.get(2); // this will get the third element of the vector v4, which is 3
    match third2 {
        Some(third2) => println!("The third element is {third2}"), // this will print "The third element is 3"
        None => println!("There is no third element."), // this will print "There is no third element."
    }




}   