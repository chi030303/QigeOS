#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    // Define the numbers
    let a: i32 = 5;
    let b: i32 = 7;

    // Calculate the sum
    let c = a + b;

    // Print the result
    println!("{} + {} = {}", a, b, c);

    0
}

