/* variable types */
fn main() {
    /* Other types are similar to other programing languages.
    However, `isize` and `usize` are platform dependent.*/
    let x: i64 = 42;
    let x32: i32 = 42;
    let xsize: isize = 42;

    // Let's print the memory size of the integer
    println!("Memory size of `x` integer: {} bits", std::mem::size_of_val(&x) * 8);
    println!("Memory size of `x32` integer: {} bits", std::mem::size_of_val(&x32) * 8);
    println!("Memory size of `xsize` integer: {} bits", std::mem::size_of_val(&xsize) * 8);

    // Floating point numbers
    let pi = "3.141592653589793238462643383279502884197";
    let pi32: f32 = std::f32::consts::PI;
    let pi64: f64 = std::f64::consts::PI;

    println!("Value of `pi`  : {pi}");
    println!("Value of `pi32`: {pi32}");
    println!("Value of `pi64`: {pi64}");
}