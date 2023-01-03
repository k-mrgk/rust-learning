use std::io::{stdin, stdout, Write};

fn fibonacci(n: usize) -> usize {
    match n {
        0 | 1 => n,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    let mut s = String::new();

    println!("Outputs the n-th Fibonacci number");
    print!("Input n > ");
    stdout().flush().unwrap();

    stdin().read_line(&mut s).expect("Failed to read line.");
    let number: usize = s.trim().parse().ok().unwrap();

    let result = fibonacci(number);

    println!("{}th Fibonacci number is {}", number, result);
}
