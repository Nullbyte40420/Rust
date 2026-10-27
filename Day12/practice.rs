fn calculate_temperature(c : f32) -> f32 {
    (c * 9.0 / 5.0) + 32.0
}

fn main(){
    let c = 23.4;
    let feranite = calculate_temperature(c);
    println!("Temperature in feranite is: {}", feranite);
    let h = String::from("Hello");
    println!("Length of string is: {}", calculate_length(&h));
    println!("{}", h);
    let s = String::from("Rust Programming");
    
    // Pehla word nikalne ke liye slice use karte hain
    let word = first_word(&s);
    
    println!("Pehla word hai: {}", word);
    
}

fn calculate_length(s: &String) -> usize{
    s.len()
}



fn first_word(s: &str) -> &str {
    // bytes() function string ke har character ko byte mein tod deta hai
    let bytes = s.as_bytes();

    // enumerate() se humein index aur byte dono milte hain
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // Jaise hi space milti hai, shuru se lekar wahan tak ka slice return kar do
            return &s[0..i];
        }
    }

    // Agar space na mile, toh poori string hi return kar do
    &s[..]
}