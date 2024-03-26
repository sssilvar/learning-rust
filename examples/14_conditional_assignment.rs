/* Conditional assignment in Rust.

Variables can be conditionally initialized in Rust using the `if` expression.
*/
fn main() {
    let is_european: bool = false;
    let frequency: i32 = if is_european { 50 } else { 60 };
    println!("Frequency: {frequency} Hz");
}