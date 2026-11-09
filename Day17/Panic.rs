// panic! is for unrecoverable errors, when something bad happened and there is nothing we can do about it
// uncomment ONE panic at a time (the program stops at the first panic, so the lines after it never run)

fn main() {
    println!("program started");

    // there are two ways a panic happens:
    // 1. we take an action that causes a panic (like accessing a vector past the end)
    // 2. we call the panic! macro ourselves

    // ---------- way 2: calling panic! directly ----------

    // this prints our message, the file:line:column where it panicked, then the program quits
    // panic!("crash and burn");
    // output looks like: thread 'main' panicked at src/main.rs:LINE:COL: crash and burn
    // the numbers after the filename are line and column number, so we know exactly where it happened

    // ---------- way 1: an action that causes a panic ----------

    let v = vec![1, 2, 3];

    // v has only 3 elements (index 0, 1, 2), so index 99 does not exist
    // there is no correct element rust could return here, so it panics
    // let _x = v[99];
    // output: index out of bounds: the len is 3 but the index is 99

    // in C this would be a "buffer overread" (undefined behaviour), you get whatever garbage is in the memory next to the array
    // this can become a security vulnerability, rust refuses to continue instead, which keeps us safe

    // for comparison, this does NOT panic, get returns None for a bad index (chapter 8)
    println!("get(99) gives: {:?}", v.get(99));

    // ---------- backtrace ----------

    // a backtrace is the list of all functions that were called to reach the panic
    // run the program like this to see it (set RUST_BACKTRACE to any value except 0):
    //   Linux / Mac:   RUST_BACKTRACE=1 cargo run
    //   Windows (PowerShell):   $env:RUST_BACKTRACE=1; cargo run
    // how to read it: start from the TOP and keep going down until you see a file YOU wrote
    // that is the place where the problem started
    // lines above it = code that our code called (std library etc.), lines below it = code that called our code
    // debug symbols must be on for this, which they are by default with cargo run (without --release)

    // ---------- unwinding vs aborting ----------

    // by default when a panic happens rust "unwinds": it walks back up the stack and cleans up the data of each function
    // this is a lot of work, so we can choose to abort instead, which ends the program immediately without cleanup
    // (the operating system cleans up the memory then, and the binary becomes smaller)
    // to do that we add this in Cargo.toml (not in main.rs):
    //   [profile.release]
    //   panic = 'abort'

    println!("program finished without panic");
}