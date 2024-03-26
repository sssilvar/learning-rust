/* Bitwise operations */
fn main() {
    let a: u8 = 0b_0100_1010;
    let b: u8 = 0b_1100_1010;

    // Bitwise AND
    println!("AND");
    println!("a     = {a:#010b}");
    println!("b     = {b:#010b}");
    println!("------------------");
    println!("a & b = {result:#010b}", result = a & b);

    // Bitwise OR
    println!("\nOR");
    println!("a     = {a:#010b}");
    println!("b     = {b:#010b}");
    println!("------------------");
    println!("a | b = {result:#010b}", result = a | b);

    // Bitwise NOT
    println!("\nNOT");
    println!("a      = {a:#010b}", a = a);
    println!("!a     = {result:#010b}", result = !a);

    // Shift left
    println!("\nShift left");
    println!("a      = {a:#010b}", a = a);
    println!("a << 1 = {result:#010b}", result = a << 1);

    // The rest of the bitwise operations are similar to the above
}