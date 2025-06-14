# Overview

This project is a command-line application written in Rust to generate Fibonacci sequences. It demonstrates handling arbitrarily large numbers and basic file output.

The purpose of this software is to explore Rust's capabilities for numerical computation, command-line argument parsing, and file system interactions. It serves as a practical exercise in learning Rust syntax and common library usage.

[Software Demo Video](http://youtube.link.goes.here)

# Development Environment

The software was developed using Rust and Cargo.

To run this software use two command:

For command line output:
    cargo run -- --terms 100 

For file output:
    cargo run -- --terms 50 --output fib.txt


Creats used:
- `num-bigint`: For handling large integers, necessary for Fibonacci numbers
- `num-traits`: Provides traits for numeric types, used here for `Zero` and `One` instances.

# Useful Websites

{Make a list of websites that you found helpful in this project}

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Crates.io](https://crates.io/)
- [Dev.to](https://dev.to/kelvinkirima014/generating-the-nth-fibonacci-in-rust-238k)


# Future Work

- Implement more robust command-line argument parsing, possibly using a crate like `clap`.
- Add more comprehensive error handling, especially for input parsing and file operations.
- Include unit tests for the Fibonacci generation logic and argument parsing.