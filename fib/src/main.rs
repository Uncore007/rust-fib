// Add BigUnit because we want to handle large Fibonacci numbers
use num_bigint::BigUint;
// Add num_traits for Zero and One traits
use num_traits::{Zero, One};
use std::env;
// Add std::fs and std::io for file operations
// Add std::io::Write for writing to files
use std::fs::File;
use std::io::{self, Write};

/// A struct to hold a Fibonacci sequence of arbitrary size.
struct Fibonacci {
    sequence: Vec<BigUint>,
}

impl Fibonacci {
    /// Generate a new Fibonacci sequence with `n` terms.
    fn new(n: usize) -> Self {
        let mut seq = Vec::with_capacity(n);
        // Add zero to start
        seq.push(Zero::zero());
        if n > 1 {
            // Add one to start
            seq.push(One::one());
            // Generate the rest of the sequence
            // Use BigUint for arbitrary precision
            for i in 2..n {
                let next = &seq[i - 1] + &seq[i - 2];
                seq.push(next);
            }
        }
        // Return the Fibonacci struct with the generated sequence
        Fibonacci { sequence: seq }
    }

    /// Write the sequence to a file, one number per line.
    fn write_to_file(&self, path: &str) -> io::Result<()> {
        let mut file = File::create(path)?;
        // Write each number in the sequence to the file
        for num in &self.sequence {
            writeln!(file, "{}", num)?;
        }
        Ok(())
    }

    /// Format the sequence as a comma-separated string.
    fn to_csv(&self) -> String {
        self.sequence
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }
}

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // Default values
    let mut terms: usize = 10;
    // Optional output path
    let mut output_path: Option<String> = None;

    // Simple argument parsing
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--terms" => {
                if i + 1 < args.len() {
                    terms = args[i + 1].parse().unwrap_or(10);
                    i += 1;
                }
            }
            "--output" => {
                if i + 1 < args.len() {
                    output_path = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    // Cap at u128 if desired, or keep arbitrary precision
    let fib = Fibonacci::new(terms);

    // If an output path is provided, write to that file; otherwise, print to stdout
    if let Some(path) = output_path {
        match fib.write_to_file(&path) {
            Ok(_) => println!("Sequence written to {}", path),
            Err(e) => eprintln!("Failed to write file: {}", e),
        }
    } else {
        // Print the sequence to stdout
        println!("Fibonacci sequence ({} terms):", terms);
        println!("{}", fib.to_csv());
    }
}