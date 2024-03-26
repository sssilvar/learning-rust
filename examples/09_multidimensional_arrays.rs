/* Multidimensional arrays in Rust */
fn main() {
    let img: [[u8; 3]; 2] = [
        [255, 0, 0],
        [0, 255, 0]
    ];

    let pixel = img[1][1];
    println!("Pixel at (1, 1) = {:?}", pixel);

    // Print all elements Using iter.enumerate()
    for (i, row) in img.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            println!("img[{}][{}] = {}", i, j, col);
        }
    }
}