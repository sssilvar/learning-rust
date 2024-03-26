/* Arrays in Rust.
Arrays in Rust are fixed-size collections of elements of the same type stored in contiguous memory.
The size of an array is part of its type signature, like this: [T; N]. The type T is the type of
the elements in the array, and N is a compile-time constant that represents the number of elements
in the array.
 */
fn main() {
    let mut letters = ['a', 'b', 'c', 'd', 'e'];

    // Modifying an element
    letters[0] = 'A';

    let first_letter = letters[0];
    let last_letter = letters[letters.len() - 1];

    println!("First letter: {first_letter}, Last letter: {last_letter}");

    // Specifying type
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    // Print all elements in the array
    for i in 0..numbers.len() {
        println!("numbers[{}] = {}", i, numbers[i]);
    }

    // Using the `iter` method with `enumerate`
    println!("\nOutput using the `iter().enumerate()` method:");
    for (i, number) in numbers.iter().enumerate() {
        println!("numbers[{}] = {}", i, number);
    }

}