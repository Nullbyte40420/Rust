
// 1. OPTION<T> DEMO: 'Some' aur 'None' kaise bante hain


fn main() {
    // Rust me 'Option' aur unke variants ('Some', 'None') by default available hote hain.
    
    let some_number: Option<i32> = Some(5);
    let some_char: Option<char> = Some('a');
    
    // Jab value Khali/Absent ho, toh hum 'None' use karte hain.
    // Compiler ko batana padta hai ki agar value aati toh wo kis type ki hoti.
    let absent_number: Option<i32> = None;

    println!("some_number: {:?}", some_number);
    println!("absent_number: {:?}", absent_number);


    // 2. ERROR CONCEPTION: Direct addition kyun nahi hota?

    let x: i8 = 10;
    let y: Option<i8> = Some(5);

    // UNCOMMENTING THIS WILL CAUSE COMPILE ERROR:
    // let sum = x + y; 
    // Error: cannot add `Option<i8>` to `i8`
    // Reason: Rust chahta hai ki pehle tum check karo ki 'y' ke andar 
    // sach me value hai (Some) ya fir wo khali hai (None).


    // 3. SAFE HANDLING: Option se value kaise nikalte hain


    // Method A: Using 'match' (Sabse safe aur standard tareeqa)
    let result = match y {
        Some(value) => x + value,
        None => x, // Agar None hua toh default value (x) return kar di
    };
    println!("Match ke zariye sum: {}", result);

    // Method B: Using 'if let' (Jab sirf 'Some' ka case handle karna ho)
    if let Some(val) = y {
        println!("y ke andar value hai: {}", val);
    } else {
        println!("y khali hai (None)!");
    }

    // Method C: Using 'unwrap_or' (Default value set karne ke liye)
    // Agar 'y' None hota, toh yeh default value 0 pick kar leta.
    let safe_sum = x + y.unwrap_or(0);
    println!("unwrap_or ke zariye sum: {}", safe_sum);


    // 4. REAL-WORLD EXAMPLE: Function returning Option

    let student_1 = find_student_score("Rahul");
    let student_2 = find_student_score("Amit");

    print_score("Rahul", student_1);
    print_score("Amit", student_2);
}

// Function jo kisi student ka score dhoondti hai
fn find_student_score(name: &str) -> Option<i32> {
    if name == "Rahul" {
        Some(95) // Rahul mil gaya, score de diya
    } else {
        None     // Koi aur student database me nahi hai, toh None de diya
    }
}

fn print_score(name: &str, score: Option<i32>) {
    match score {
        Some(s) => println!("{} ka score hai: {}", name, s),
        None => println!("{} ka record nahi mila!", name),
    }
}