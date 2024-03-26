/* Conditional execution blocks in Rust */
fn main() {
    let mut age: i32;

    loop {
        age = ask_age();
        if age > 18 {
            println!("You are an adult ({age} yo)");
        } else if age == 18 {
            println!("Congrats! You've just become an adult!")
        } else {
            println!("You are a minor ({age} yo)");
        }
    }
}

fn input(message: &str) -> String {
    use std::io::{self, Write};

    print!("{message}");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

fn exit(message: String, code: i32) -> ! {
    eprintln!("[Error] {message}");
    std::process::exit(code);
}

fn ask_age() -> i32 {
    let data = input("Enter your age (negative to exit): ");

    // Try to parse otherwise print 'invalid value' and exit
    data.parse::<i32>().unwrap_or_else(|_| exit(format!("Invalid value: {data}"), 1))
}
