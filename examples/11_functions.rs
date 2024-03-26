/** Functions in Rust */
fn main() {
    let x = 10;
    let y = 3;
    let z = add(x, y);
    println!("add({x}, {y}) = {z}");

    // If we try another type, we get a compile-time error
    // `error[E0308]: mismatched types`
    // let z2 = add(x, 3.0);

    let (x, x_square) = value_square(10);
    println!("value_square(10) = ({x}, {x_square})");

    // Also, we can pass the result directly to the println! macro using the {x:?} syntax
    println!("value_square(20) = {:?}", value_square(20));
}

fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn value_square(x: i32) -> (i32, i32) {
    return (x, x * x);
}