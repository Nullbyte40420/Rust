fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn main() {
    let c = 25.0;
    let f = 98.6;

    println!("{c}°C = {:.2}°F", celsius_to_fahrenheit(c));
    println!("{f}°F = {:.2}°C", fahrenheit_to_celsius(f));
}