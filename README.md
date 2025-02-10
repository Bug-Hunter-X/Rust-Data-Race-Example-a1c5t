# Rust Data Race Example

This repository demonstrates a simple data race in Rust.  The `bug.rs` file contains code that creates two mutable references to the same variable, leading to undefined behavior when both references modify the variable concurrently. The `bugSolution.rs` file shows how to resolve this issue using techniques such as mutexes or other synchronization primitives. 

## How to reproduce the bug
1. Clone the repository.
2. Navigate to the root directory.
3. Run the `bug.rs` file using Rust's compiler and runner: `rustc bug.rs && ./bug` 

## How to solve the bug
The solution involves using appropriate synchronization mechanisms to prevent data races. Refer to `bugSolution.rs` for a possible solution.