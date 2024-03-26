/* Challenge 1:

Compute the arithmetic average of three variables. Fix and complete  the code below to make it work.
 */
fn main() {
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    let average = (a as f64 + b + c as f64) / 3.0;

    assert_eq!(average, 45.1);
    println!("Test passed!");
}