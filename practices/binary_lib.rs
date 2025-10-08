1. Binary Crate

Produces an executable program (.exe, .out, etc.).

Has a main() function as the entry point.

File: src/main.rs

Example:

fn main() {
    println!("Hello, world!");
}


When you run cargo run, it builds and runs the binary.

👉 Think: A full program you can run.

2. Library Crate

Produces reusable code (no executable).

No main() function.

File: src/lib.rs

Example:

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}


When another crate depends on this, it can use:

use my_crate::add;


👉 Think: A toolbox of functions, structs, traits, etc.

Yes, there **is a rule in Rust**:  
For a **library crate**, the file named lib.rs is **always the main entry point**.

### Why?

- This is defined by **Rust’s build system (Cargo)**.
- When you create a library crate (with `cargo new --lib my_crate`), Cargo expects your main code to be in lib.rs.
- If you create a binary crate (with `cargo new my_crate`), Cargo expects your main code to be in `src/main.rs`.

### You don’t need to specify this anywhere—Cargo does it automatically.

---

**Summary:**  
- For libraries: lib.rs is always the entry point.
- For binaries: `src/main.rs` is always the entry point.
- This is a convention built into Rust and Cargo.

**You just follow the convention—no need to configure it!**