/* Boolean type in Rust */
fn main() {
    let t = true;
    let f: bool = false;

    // Logical operations allowed in this type
    println!("t AND f = {result}", result = t && f);
    println!("t OR f  = {result}", result = t || f);
    println!("NOT t   = {result}", result = !t);

    // Also support bitwise operations
    println!("\nBitwise operations");
    println!("t AND f = {result}", result = t & f);
    println!("t OR f  = {result}", result = t | f);
    println!("NOT t   = {result}", result = !t);
    println!("t XOR f = {result}", result = t ^ f);

    // These two last should be true
    println!("true XNOR true = {result}", result = !(true ^ true));
    println!("false XNOR false = {result}", result = !(false ^ false));

    // Comparison operators
    println!("\nComparison operators");
    println!("t == f  = {result}", result = t == f);
    println!("t != f  = {result}", result = t != f);
    println!("t > f   = {result}", result = t > f);
    println!("t < f   = {result}", result = t < f);

}