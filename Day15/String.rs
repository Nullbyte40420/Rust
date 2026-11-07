fn main() {
    // rust has only one string type in the core language, which is str (usually seen as &str, a borrowed string slice)
    // string literals are &str, they are stored inside the program's binary
    // String is from the standard library, it is growable, mutable, owned and UTF-8 encoded

    // ---------- creating a String ----------

    // this will create a new empty String
    let mut s = String::new();
    println!("empty s: '{s}'");

    // to_string works on any type that implements Display, string literals do
    let data = "initial contents"; // this is a &str
    let s1 = data.to_string(); // this will create a String from the &str
    let s2 = "initial contents".to_string(); // works directly on a literal too

    // String::from does the same thing as to_string, it's just a style choice
    let s3 = String::from("initial contents");
    println!("{s1} | {s2} | {s3}");

    // strings are UTF-8 so any language works
    let hello_hi = String::from("नमस्ते");
    let hello_jp = String::from("こんにちは");
    let hello_ru = String::from("Здравствуйте");
    println!("{hello_hi} {hello_jp} {hello_ru}");

    // ---------- updating a String ----------

    // push_str adds a string slice at the end, and it does NOT take ownership of the argument
    s.push_str("foo");
    s.push_str("bar");
    println!("after push_str: {s}"); // foobar

    let mut a = String::from("foo");
    let b = "bar";
    a.push_str(b);
    println!("b is still usable: {b}"); // works because push_str only borrowed b

    // push adds a single char, notice the single quotes (char), not double quotes (&str)
    let mut c = String::from("lo");
    c.push('l');
    println!("after push: {c}"); // lol

    // ---------- concatenation with + ----------

    let x1 = String::from("Hello, ");
    let x2 = String::from("world!");
    // + calls add(self, s: &str) -> String
    // self has no &, so x1 is MOVED into add and can't be used after this
    // &x2 is &String but add wants &str, so rust does deref coercion (&x2 becomes &x2[..])
    let x3 = x1 + &x2;
    println!("{x3}");
    // println!("{x1}"); // error: x1 was moved
    println!("x2 still works: {x2}"); // x2 was only borrowed

    // + gets ugly with many strings
    let t1 = String::from("tic");
    let t2 = String::from("tac");
    let t3 = String::from("toe");
    let ugly = t1 + "-" + &t2 + "-" + &t3; // t1 is moved here, t2 and t3 are fine
    println!("{ugly}");

    // format! works like println! but returns a String instead of printing
    // and it only uses references, so nothing is moved
    let f1 = String::from("tic");
    let f2 = String::from("tac");
    let f3 = String::from("toe");
    let nice = format!("{f1}-{f2}-{f3}");
    println!("{nice} (f1, f2, f3 are all still usable: {f1} {f2} {f3})");

    // ---------- indexing into Strings ----------

    // this does NOT compile, so it is commented
    // let s_hi = String::from("hi");
    // let h = s_hi[0]; // error: the type `str` cannot be indexed by `{integer}`

    // reasons rust doesn't allow it:
    // 1. String is a wrapper over Vec<u8>, and one char can take more than 1 byte
    // 2. s[0] would give a byte (104 for 'h'), not the char, which is not what people expect
    // 3. indexing is expected to be O(1), but to find the n-th char rust must walk from the start

    // ---------- internal representation ----------

    let hola = String::from("Hola");
    println!("Hola len: {}", hola.len()); // 4 bytes, each letter is 1 byte

    let zdravstvuyte = String::from("Здравствуйте");
    println!("Здравствуйте len: {}", zdravstvuyte.len()); // 24, not 12, each letter is 2 bytes

    // same word 3 ways: bytes, scalar values (char), grapheme clusters
    let namaste = String::from("नमस्ते");
    println!("namaste bytes: {}", namaste.len()); // 18 bytes
    println!("namaste chars: {}", namaste.chars().count()); // 6 chars (some are not letters on their own, like diacritics)
    // grapheme clusters (4 letters: न म स् ते) are not in the standard library, we need a crate for that

    // ---------- slicing strings ----------

    // slicing with a range is allowed but it works on BYTES
    let hello = "Здравствуйте";
    let part = &hello[0..4]; // first 4 bytes, each letter is 2 bytes, so this is "Зд"
    println!("slice: {part}");

    // this compiles but PANICS at runtime, because 0..1 cuts the first letter in the middle of its bytes
    // error: byte index 1 is not a char boundary
    // let bad = &hello[0..1];

    // ---------- iterating over strings ----------

    // chars gives each unicode scalar value (char)
    for ch in "Зд".chars() {
        println!("{ch}"); // З then д
    }

    // bytes gives each raw byte
    for byte in "Зд".bytes() {
        println!("{byte}"); // 208 151 208 180
    }

    // ---------- some useful methods ----------

    let text = String::from("I like rust, rust is fun");
    println!("contains rust? {}", text.contains("rust")); // true
    println!("replaced: {}", text.replace("rust", "Rust")); // replaces every match, returns a new String
    println!("original is untouched: {text}");
}