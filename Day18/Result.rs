use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

// Result is an enum with two variants, Ok(T) and Err(E)
// T is the type of the success value, E is the type of the error value (these are generics, chapter 10)
// like Option, Result and its variants are in the prelude, so we write Ok and Err directly

// ---------- 1. match on Result, panic on any error ----------

fn open_or_panic(path: &str) -> File {
    // File::open returns Result<File, io::Error>, because opening a file can fail
    let file_result = File::open(path);

    match file_result {
        Ok(file) => file, // success, we take the file handle out of Ok
        Err(error) => panic!("Problem opening the file: {error:?}"),
    }
}

// ---------- 2. matching on different kinds of errors ----------

fn open_or_create(path: &str) -> File {
    match File::open(path) {
        Ok(file) => file,
        // error.kind() gives an ErrorKind enum, so we can match on WHY it failed
        Err(error) => match error.kind() {
            // file not found, so we try to create it (and creating can fail too, so another match)
            ErrorKind::NotFound => match File::create(path) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            // any other reason (like no permission), we still panic
            _ => panic!("Problem opening the file: {error:?}"),
        },
    }
}

// same logic as above using closures and unwrap_or_else, no match needed
// closures are chapter 13, so don't worry if this looks strange right now, just come back later
fn open_or_create_closure(path: &str) -> File {
    File::open(path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(path).unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    })
}

// ---------- 3. propagating errors (returning the error to the caller) ----------

// the function does not decide what to do on error, it returns the Err to the caller
// the return type Result<String, io::Error> means: success gives a String, failure gives an io::Error
fn read_username_manual(path: &str) -> Result<String, io::Error> {
    let username_file_result = File::open(path);

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e), // return early out of the whole function with the error
    };

    let mut username = String::new();

    // read_to_string also returns a Result, because reading can fail even if open worked
    // this is the last expression, so no return keyword needed
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// ---------- 4. the ? operator ----------

// ? after a Result: if it is Ok, the value inside is given to us and the function continues
// if it is Err, the error is returned from the WHOLE function (same as the match + return Err(e) above)
// ? also passes the error through the From trait, so it can convert it into the function's error type
fn read_username_question(path: &str) -> Result<String, io::Error> {
    let mut username_file = File::open(path)?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// we can chain method calls right after the ?
fn read_username_chained(path: &str) -> Result<String, io::Error> {
    let mut username = String::new();

    File::open(path)?.read_to_string(&mut username)?;

    Ok(username)
}

// the standard library already has a function for this exact thing
fn read_username_fs(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

// ---------- 5. ? on Option ----------

// ? works on Option too, but only inside a function that returns Option
// if the value is None, the function returns None early, if it is Some we get the value inside
fn last_char_of_first_line(text: &str) -> Option<char> {
    // lines() gives an iterator over lines, next() gives the first one (None if the text is empty)
    // chars().last() gives the last char of that line (None if the line is empty)
    text.lines().next()?.chars().last()
}

// we can NOT mix them: ? on a Result inside a function that returns Option (or the opposite) does not compile
// to convert we use .ok() on a Result or .ok_or() on an Option

// ---------- main ----------

// main normally returns (), and then we can not use ? inside it
// error: the `?` operator can only be used in a function that returns `Result` or `Option`
// so we make main return Result<(), Box<dyn Error>>
// Box<dyn Error> can be read as "any kind of error" for now (trait objects are chapter 18)
// if main returns Ok(()) the program exits with code 0, if it returns Err the exit code is non zero
fn main() -> Result<(), Box<dyn Error>> {
    // hello.txt will be created in the folder where we run cargo run, if it doesn't exist
    let file = open_or_create("hello.txt");
    println!("opened or created: {:?}", file);

    let file2 = open_or_create_closure("hello.txt");
    println!("closure version: {:?}", file2);

    // these return Result, so we print them with {:?}, we can see Ok("...") or Err(...)
    // write some text in hello.txt to see the username, and rename the file to see the Err
    println!("manual: {:?}", read_username_manual("hello.txt"));
    println!("question mark: {:?}", read_username_question("hello.txt"));
    println!("chained: {:?}", read_username_chained("hello.txt"));
    println!("fs: {:?}", read_username_fs("hello.txt"));

    // a file that does not exist, so we get Err(...) back and the program continues, no panic
    println!("missing file: {:?}", read_username_question("nope.txt"));

    println!("last char: {:?}", last_char_of_first_line("hello\nworld")); // Some('o')
    println!("empty text: {:?}", last_char_of_first_line("")); // None
    println!("blank first line: {:?}", last_char_of_first_line("\nhi")); // None

    // ---------- shortcuts for panic on error ----------

    // unwrap: gives the value if Ok, panics if Err (with a default message)
    // let _f = File::open("nope.txt").unwrap();
    // output: called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, ... }

    // expect: same as unwrap but WE choose the panic message, so it is easier to track down later
    // in real code expect is preferred over unwrap, with a message about why we thought it would succeed
    // let _f = File::open("nope.txt").expect("nope.txt should be included in this project");

    // ? inside main, works because main returns Result now
    // if nope.txt is missing, main returns the Err and the program exits with a non zero code
    // let _f = File::open("nope.txt")?;

    // we still call open_or_panic once, uncomment the line below with a missing file to see the panic
    // let _f = open_or_panic("nope.txt");
    let _f = open_or_panic("hello.txt"); // exists, so no panic

    Ok(()) // main now has to return a Result, so we end with Ok(())
}