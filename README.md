
# Parallel Operations Library

This Rust library provides a set of parallel binary operations such as addition, subtraction, multiplication, and more. The operations are designed to be computed concurrently using threads, leveraging the power of multiple CPU cores for faster performance.

## Features

- Perform binary operations (e.g., addition, subtraction, multiplication) concurrently using threads.
- Automatically adjusts the number of threads based on the available CPU cores.
- Optimized for large datasets, ideal for high-performance computing.
- Simple and flexible API to support any binary operation on vectors of numeric types.

## Getting Started

To use the `parallel_operations` crate in your project, follow these steps:

```bash
cargo add parallel_operations
```

```rust 
use parallel_operations::parallel_binary_operation;

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    
    // Parallel addition
    let addition_result = parallel_binary_operation(data.clone(), |a, b| a + b);
    println!("Addition result: {}", addition_result);
}
```
