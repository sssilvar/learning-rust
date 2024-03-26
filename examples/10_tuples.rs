/* Tuples in Rust.
    * Tuples are a way to group multiple values into a single compound value.
    * They are fixed-size and can contain elements of different types.
    * Tuples are useful for returning multiple values from functions.
 */
fn main() {
    let stuff: (u8, f32, char, &str) = (42, 3.14, 'A', "Hello, world!");

    // Accessing elements
    let number = stuff.0;
    let pi = stuff.1;
    let letter = stuff.2;
    let message = stuff.3;

    println!("Number: {}", number);
    println!("Pi: {}", pi);
    println!("Letter: {}", letter);
    println!("Message: {}", message);

    // Tuples as in Python can be used to assign multiple variables at once:
    let (a, b, c, d) = stuff;
    println!("\nUsing tuple destructuring:");
    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}", d);   
}