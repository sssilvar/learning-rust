/* Hello world app

The first must-do app in any language you start to learn.
To compile you can use cargo as follows:

cargo run --color=always --package learning-rust --example 00_hello_world
*/
fn main() {
    let who = "World";
    let mut population: i64 = 7_800_000_000;
    let mut year = 2021;
    println!("Hello, {who}! We were {population} in {year}!");

    // Let's simulate a population growth
    year += 1;
    let growth_rate = 0.01;
    population += (population as f64 * growth_rate) as i64;

    println!("Hello again! We are {population} in {year}!");
}
