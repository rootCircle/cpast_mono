use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let number: f64 = input.trim().parse().expect("Please enter a valid number");
    let square = number * number;
    println!("{}", square);
}
