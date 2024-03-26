/* Loops in Rust.

There exist the following types of loops in Rust:
    * loop: an infinite loop.
    * while: a loop that repeats while a condition is true.
    * for: a loop that iterates over a range or an iterator.
 */
fn main() {
    // Infinite loop
    let mut counter = 0;

    println!("Using loop:");
    loop {
        counter += 1;
        println!("Counter: {}", counter);
        if counter == 10 {
            break;
        }
    }

    // While loop
    println!("\nUsing while loop:");
    let mut counter = 0;
    while counter < 10 {
        counter += 1;
        println!("Counter: {}", counter);
    }

    // For loop
    println!("\nUsing for loop:");
    for i in 0..10 {
        println!("Counter: {}", i);
    }

    // For loop with a range
    // rev() is a method that reverses the range
    println!("\nUsing for loop with a range:");
    for i in (0..10).rev() {
        println!("Counter: {}", i);
    }
}