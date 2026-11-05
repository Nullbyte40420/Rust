// vectors can only hold one type of data, so if we want different types we use an enum
// all the variants of an enum are considered the same type (the enum itself)
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // first collection type is a Vec<T> or known as vector, which is a resizable array

    // vector can only hold one type of data, so we need to specify the type of data it will hold

    // this will create a empty vector of type i32, which is a signed 32-bit integer
    // Note that we added a type annotation here. Because we aren't inserting any values into this vector, Rust doesn't know what type of elements we want to store.
    let mut v: Vec<i32> = Vec::new(); // this will create a empty vector of type i32, which is a signed 32-bit integer

    // vectors are implemented using generics, so we can create a vector of any type, like this:
    let mut v2: Vec<String> = Vec::new(); // this will create a empty vector of type String

    // Rust conveniently provides the vec! macro to create a new vector that contains initial values, like this:
    let v3 = vec![1, 2, 3]; // this will create a vector of type i32

    // updating a vector is done using the push method, which adds an element to the end of the vector, like this:
    v2.push(String::from("Hello")); // this will add "Hello" to the end of the vector v2
    v.push(1); // this will add 1 to the end of the vector v

    // {:?} works on vectors because Vec<T> implements Debug (if T implements Debug)
    println!("v: {:?}, v2: {:?}, v3: {:?}", v, v2, v3);

    // accessing elements in a vector is done using the two ways, using indexing or using the get method, like this:
    let v4 = vec![1, 2, 3, 4, 5];
    // indexing is done using the square brackets, like this:
    let third: &i32 = &v4[2]; // this will get the third element of the vector v4, which is 3
    println!("The third element is {third}"); // this will print "The third element is 3"

    // using the get method is done using the get method, like this:
    // the get method returns an Option<&T>, which is an enum that can be either Some(&T) or None
    let third2: Option<&i32> = v4.get(2); // this will get the third element of the vector v4, which is 3
    match third2 {
        Some(value) => println!("The third element is {value}"), // this runs here because v4 has 5 elements
        None => println!("There is no third element."), // this would only run if the index was out of range
    }

    // ---------- index out of range: [] vs get ----------

    // this will compile fine but PANIC at runtime, because index 100 does not exist in v4 (only 5 elements)
    // so i keep it commented, uncomment it to see the panic
    // let a = &v4[100];

    // get does not panic, it returns None if the index is out of range
    let b = v4.get(100); // this will be None
    match b {
        Some(value) => println!("Element at 100 is {value}"),
        None => println!("Index 100 is out of range, no panic"), // this will print
    }
    // use [] when you WANT the program to crash on a bad index (it would be a bug)
    // use get when a bad index can happen normally (like user input) and you want to handle it

    // ---------- borrow checker with vectors ----------

    // this code does NOT compile, so it is commented
    // let mut v5 = vec![1, 2, 3, 4, 5];
    // let first = &v5[0];    // immutable borrow starts here
    // v5.push(6);            // mutable borrow, error because `first` is still alive
    // println!("The first element is: {first}"); // immutable borrow is used here
    // reason: push may need new memory and copy everything (vector keeps values next to each other),
    // then `first` would point to freed memory. borrow checker stops that.

    // this works, because first's last use is before the push
    let mut v5 = vec![1, 2, 3, 4, 5];
    let first = &v5[0];
    println!("The first element is: {first}"); // last use of first
    v5.push(6); // now mutable borrow is fine
    println!("v5 after push: {:?}", v5);

    // ---------- iterating over a vector ----------

    let v6 = vec![100, 32, 57];
    // &v6 gives immutable reference to each element, so i is &i32
    for i in &v6 {
        println!("{i}");
    }

    let mut v7 = vec![100, 32, 57];
    // &mut v7 gives mutable reference to each element, so i is &mut i32
    for i in &mut v7 {
        // we need * (dereference) to reach the actual value behind the reference before using +=
        *i += 50;
    }
    println!("v7 after adding 50: {:?}", v7); // [150, 82, 107]
    // we can not push or remove inside these loops, because the loop already holds a borrow of the whole vector

    // ---------- using an enum to store multiple types ----------

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // match makes sure we handle every variant, compiler will complain if we miss one
    for cell in &row {
        match cell {
            SpreadsheetCell::Int(n) => println!("int: {n}"),
            SpreadsheetCell::Float(f) => println!("float: {f}"),
            SpreadsheetCell::Text(s) => println!("text: {s}"),
        }
    }

    // ---------- pop ----------

    let mut v8 = vec![1, 2, 3];
    // pop removes the last element and returns Option<T>, because the vector can be empty (then it gives None)
    let last = v8.pop();
    println!("popped: {:?}, vector now: {:?}", last, v8); // popped: Some(3), vector now: [1, 2]

    // ---------- dropping a vector ----------

    {
        let v9 = vec![1, 2, 3, 4];
        println!("inside scope: {:?}", v9);
    } // <- v9 goes out of scope here, the vector and all its elements are freed
    // println!("{:?}", v9); // this would give error, v9 does not exist outside the curly brackets
}