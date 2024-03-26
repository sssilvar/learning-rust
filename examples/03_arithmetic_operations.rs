/* Arithmetic operations in Rust */
fn main() {
    // Addition
    let x = 10;
    let y = 3;
    let z = x + y;
    println!("{x} + {y} = {z}");

    // Subtraction
    let z = x - y;
    println!("{x} - {y} = {z}");

    // Multiplication
    let z = x * y;
    println!("{x} * {y} = {z}");

    // Division
    let z = x / y;
    println!("{x} / {y} = {z} (integer division if type is automatically inferred)");

    // Division with floating point numbers by casting
    // We can limit the number of decimal places by using `:.3` in the format string
    let z = x as f64 / y as f64;
    println!("{x} / {y} = {z:.3} (floating point division by casting to f64)");

    // Modulus
    let z = x % y;
    println!("{x} % {y} = {z}");
}