use std::io;

const A: f64 = 32f64;
const B: f64 = 5f64 / 9f64;

fn f2c(fahrenheit: f64) -> f64 {
    return (fahrenheit - A) * B;
}

fn c2f(celsius: f64) -> f64 {
    return (celsius / B) + A;
}

fn main() {
    println!("Enter a number: ");
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

    let number: f64 = buffer.trim().parse().expect("Not a valid number");

    println!("{}℉\t->\t{}℃", number, f2c(number));
    println!("{}℃\t->\t{}℉", number, c2f(number));
}
