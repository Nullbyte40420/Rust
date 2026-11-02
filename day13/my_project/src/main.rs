// crate root file of the binary crate
use crate::garden::vegetables::Asparagus;

// this line tells the compiler to include the code from the `garden` module
pub mod garden; // Declare the `garden` module, which is defined in `garden.rs` or `garden/mod.rs` 

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}