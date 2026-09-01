fn main(){
    let x = 5;//  this is an integer literal
    let y:i32 = 5; // this is an integer literal with type annotation
    // there are different types of type notation for integer types

    // signed and unsigned integer types
    let a:u8 = 255; // this is an unsigned 8 bit integer type
    // used for positive numbers and zero only
    // limits from 0 to 2^(n) - 1 where n is the number of bits used to represent the number

    let b:i8 = -128; // this is a signed 8 bit integer type
    // it used two's complement representation to represent negative numbers
    // used for both positive and negative numbers
    // limits from -2^(n-1) to 2^(n-1) - 1 inclusively where n is the number of bits used to represent the number

    // there are several more type noation 
    // u16, i16, u32, i32, u64, i64, u128, i128, usize, isize
    let c:usize = 100; // this is an unsigned integer type in this usize take system architecture into account like mine is 64bit so it can hold values from 0 to 2^(64) - 1

    // by default type anotation for integer literals is i32 if not specified otherwise
    let a = 5; // this is an integer literal with default type annotation of i32


    // different literals for integer types
    let d = 100; // 100 is a integer literal in decimal notation(base 10) 
    let x = 10_000; // underscore is used for better readability of large numbers in decimal notation(base 10)

    // binary notation(base 2)
    let e = 0b1111_0000; // this is a binary literal with value of 240 in decimal notation(base 10) (1111_0000 in binary is equal to 240 in decimal)

    // octal notation(base 8)
    let f = 0o77; // this is an octal literal with value of 63 in decimal notation(base 10) (77 in octal is equal to 63 in decimal)

    // hexadecimal notation(base 16)
    let g = 0xFF; // this is a hexadecimal literal with value of 255 in decimal notation(base 10) (FF in hexadecimal is equal to 255 in decimal)

    // byte(u8) notation => singleASCII character
    let h = b'A'; // this is a byte literal with value of 65 in decimal notation(base 10) (A in ASCII is equal to 65 in decimal)

    // type suffix notation
    let i = 100u32; // this is a integer literal with type suffix notation of u32


    // integer overflow and underflow
    let mut j:u8 = 255; // this is an unsigned 8 bit integer type
    println!("The value of j is: {}", j); // this will print 255
    //j = j + 1; // this will cause an integer overflow and panic in debug mode and wrap around to 0 in release mode

    j = 0 ; // this is an unsigned 8 bit integer type
    //j = j - 1; // this will cause an integer underflow and panic in debug mode and wrap around to 255 in release mode


}