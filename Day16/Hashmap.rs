// HashMap is not in the prelude (vec and String are), so we need to bring it into scope ourselves
use std::collections::HashMap;

fn main() {
    // HashMap<K, V> stores a mapping of keys of type K to values of type V
    // we look up data using a key (can be any type), not an index like a vector
    // just like vectors, the data is stored on the heap, and it is homogeneous
    // (all keys must be the same type, all values must be the same type)

    // ---------- creating a hash map ----------

    // there is no macro like vec! for hash maps, so we create it with new and add stuff with insert
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // rust infers this as HashMap<String, i32> from the first insert
    println!("scores: {:?}", scores);

    // ---------- accessing values ----------

    let team_name = String::from("Blue");
    // get takes a reference to the key and returns Option<&V>, because the key may not exist
    // copied() turns Option<&i32> into Option<i32>
    // unwrap_or(0) gives the value if it is Some, otherwise gives 0
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Blue score: {score}"); // 10

    // a key that does not exist, so we get the default 0
    let missing = scores.get("Red").copied().unwrap_or(0);
    println!("Red score: {missing}"); // 0

    // iterating over a hash map is like a vector but we get (key, value) pairs
    // order is arbitrary, so it may print in a different order every time you run it
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // ---------- ownership in hash maps ----------

    // types that implement Copy (like i32) are copied into the map
    // owned types like String are MOVED into the map, and the map becomes the owner
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(field_name, field_value);
    // println!("{field_name}"); // error: field_name was moved into the map
    // println!("{field_value}"); // error: same for field_value
    println!("map: {:?}", map);

    // if we insert references, the values are NOT moved, but they must live at least as long as the map
    // (this is the lifetimes stuff from chapter 10, so for now just know it works like this)
    let key2 = String::from("language");
    let value2 = String::from("rust");
    let mut ref_map: HashMap<&String, &String> = HashMap::new();
    ref_map.insert(&key2, &value2);
    println!("ref_map: {:?}, key2 is still usable: {key2}", ref_map);

    // ---------- updating a hash map ----------
    // each unique key can have only ONE value at a time (but two keys can have the same value)
    // when a key already has a value we can: overwrite it, keep the old one, or combine old and new

    // 1. overwriting a value
    let mut overwrite: HashMap<String, i32> = HashMap::new();
    overwrite.insert(String::from("Blue"), 10);
    overwrite.insert(String::from("Blue"), 25); // same key, so the old value 10 is replaced
    println!("overwrite: {:?}", overwrite); // {"Blue": 25}

    // 2. adding only if the key is not present, using the entry api
    // entry returns an enum called Entry, which represents a value that might or might not exist
    // or_insert inserts the value only if the key is missing, and returns &mut V to the value either way
    let mut entry_map = HashMap::new();
    entry_map.insert(String::from("Blue"), 10);

    entry_map.entry(String::from("Yellow")).or_insert(50); // Yellow is missing, so it is inserted
    entry_map.entry(String::from("Blue")).or_insert(50); // Blue already exists, so nothing changes
    println!("entry_map: {:?}", entry_map); // Blue is still 10, Yellow is 50

    // 3. updating a value based on the old value (word counter)
    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();

    // split_whitespace gives an iterator over the words
    for word in text.split_whitespace() {
        // or_insert returns &mut V, so count is a mutable reference to the number inside the map
        // if the word is new it inserts 0 first
        let count = word_count.entry(word).or_insert(0);
        // count is a reference, so we need * (dereference) to change the actual value
        *count += 1;
    } // count goes out of scope at the end of each loop iteration, so the mutable borrow is safe
    println!("word_count: {:?}", word_count); // world: 2, hello: 1, wonderful: 1 (order may differ)

    // ---------- hashing function ----------
    // by default HashMap uses SipHash, which is not the fastest but resists DoS attacks on hash tables
    // if profiling shows it is too slow, we can use a different hasher (a type that implements BuildHasher)
    // we can find ready made hashers on crates.io, no need to write one from scratch
}