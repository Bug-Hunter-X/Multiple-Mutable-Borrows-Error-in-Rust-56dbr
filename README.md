# Multiple Mutable Borrows in Rust

This example demonstrates a common error in Rust related to mutable borrowing. Rust's borrow checker prevents data races by ensuring that only one mutable borrow of a variable exists at any given time.  Attempting to create multiple mutable borrows results in a compile-time error.

The `bug.rs` file showcases the problematic code. The `bugSolution.rs` file provides a corrected version.
