# Rust Raw Pointer and Vector Reallocation Bug

This repository demonstrates a potential issue when using raw pointers with Rust vectors. Modifying a vector's elements via a raw pointer after a reallocation may lead to unexpected behavior or crashes.

The `bug.rs` file contains the buggy code, showcasing the unsafe usage of a raw pointer.  The `bugSolution.rs` file provides a safer approach to achieve the intended outcome without the use of raw pointers.

## How to reproduce
1. Clone this repository.
2. Navigate to the repository's directory.
3. Compile and run `bug.rs`:  `rustc bug.rs && ./bug`
4. Observe the unexpected behavior or crash (depending on the compiler and runtime).
5. Run the corrected version in `bugSolution.rs` to see the intended behavior.

## Solution
The solution involves avoiding raw pointers and utilizing safe Rust features instead. This ensures memory safety and prevents undefined behavior. The example shown in `bugSolution.rs` demonstrates a safer approach by using indexing to modify the vector element directly.