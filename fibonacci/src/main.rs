use std::io;
use std::time::Instant;

fn fibonacci(n: usize, algo_name: &str, algo: &dyn Fn(usize) -> usize) {
    let now = Instant::now();
    let fib = algo(n);
    let elapsed_time = now.elapsed();

    println!(
        "{}th Fibonacci number is: {} ({}) - It took {} ms.",
        n,
        fib,
        algo_name,
        elapsed_time.as_millis(),
    );
}

fn iter_fibonacci(n: usize) -> usize {
    let mut a: usize = 0;
    let mut b: usize = 1;
    let mut fib: usize = 0;
    for _ in 1..n {
        fib = a + b;
        a = b;
        b = fib;
    }

    return fib;
}

fn recursive_fibonacci(n: usize) -> usize {
    if n <= 1 {
        return n;
    }

    return recursive_fibonacci(n - 1) + recursive_fibonacci(n - 2);
}

fn main() {
    let mut buffer = String::new();

    println!("Enter a number:");

    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

    let input: usize = buffer.trim().parse().expect("Not a number");

    fibonacci(input, "iterative", &iter_fibonacci);
    fibonacci(input, "recusive", &recursive_fibonacci);
}
