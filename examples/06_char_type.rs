/* char type in Rust */
fn main() {
    let letter = 'A';
    let emoji = '😀';

    // Print as HEX
    println!("Letter '{letter}'\t= {:#X}", letter as u32);
    println!("Emoji '{emoji}'\t= {:#X}", emoji as u32);
}