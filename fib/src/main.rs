// Need BigUnit for large Fibonacci numbers
use num_bigint::BigUint;
// Use traits for zero and one to initialize BigUint
use num_traits::{Zero, One};
// Import standard library for command line argument parsing
use std::env;

fn generate_fib(n: usize) -> Vec<BigUint> {
    // Initialize a vector to hold the Fibonacci sequence
    let mut seq: Vec<BigUint> = Vec::with_capacity(n);

    // Add the first number to the sequence
    seq.push(Zero::zero());
    // If n is greater than 1, add the second number
    if n > 1 {
        // Add the second number to the sequence
        seq.push(One::one());

        // Generate the Fibonacci sequence iteratively
        // Start from the third term and calculate up to n
        for i in 2..n {
            let next = &seq[i - 1] + &seq[i - 2];
            seq.push(next);
        }
    }

    seq
}

fn main() {
    // Collect command line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // Check if a number of terms is provided; default to 10 if not
    let terms: usize = if args.len() > 1 {
        match args[1].parse::<usize>() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Invalid number provided; defaulting to 10.");
                10
            }
        }
    } else {
        10
    };

    // Generate the Fibonacci sequence with the specified number of terms
    let fib_sequence = generate_fib(terms);

    // Print the Fibonacci sequence
    println!("Fibonacci sequence ({} terms):", terms);

    let output = fib_sequence
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    println!("{}", output);
}
