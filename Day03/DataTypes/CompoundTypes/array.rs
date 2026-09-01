fn main() {
    // unlike tuple array can hold only same type of values and it can't be resized at any cost at all it can't shrink or grow
    let mut arr = [1, 2, 3, 4, 5]; // array is a compound type which can hold multiple values of the same type
    let first = arr[0]; // this is indexing
    arr[0] = 10; // this is indexing and assignment
    println!("The value of first is: {}", first); // this will print 1
    let arr1: [i32; 5] = [1, 2, 3, 4, 5];
    let arr2 = [3; 5]; // this is an array of 5 elements all initialized to 3 [3, 3, 3, 3, 3]

}