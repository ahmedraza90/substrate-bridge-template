// Stack
// Temporary.
// Variables go in and out of existence as functions are called/returned.
// Freed automatically when scope ends.

// Heap
// Dynamically allocated while program runs.
// Needs explicit owner to free before program ends (otherwise → memory leak if the program runs long).

// Static / Read-only memory
// Baked into the binary when the program is compiled.
// Data is available for the entire lifetime of the program.
// You don’t allocate or free it manually — it just exists as long as the program is loaded.

// const
// The compiler inlines the value.
// That means whenever you use const NAME, the compiler replaces it with the literal value directly in the code.
// There is no memory address for a const. It doesn’t exist as a variable in memory at runtime.
// Example:

const X: i32 = 5;
fn main() {
    let a = X; // compiler rewrites this as: let a = 5;
}

// static
// compiler allocates a fixed address in the program’s binary, value is stored there once, and every access goes through that address.
// Every time you use static NAME, you are reading from program’s data segment in memory — not the stack, not the heap, but the static global data area loaded from the binary..

// 1. Where do static values live?

// When you compile a Rust program, the compiler produces a binary file (an executable).
// That binary is divided into sections. The most important ones:

// .text → the actual machine code (functions).
// .rodata → read-only data (constants, immutable statics, string literals). read-only memory of the program
// .data → writable global data (for static mut).
// .bss → space for uninitialized global variables.
