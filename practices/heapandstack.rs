// When a program terminates, ALL memory (both stack AND heap) gets completely wiped out by the operating system. Nothing in RAM survives program termination.

// When your Rust program runs:
// STACK: Contains function variables, Arc pointers, etc.
// HEAP:  Contains HashMap data, String data, Vec data, etc.

// When program terminates:
// STACK: ❌ COMPLETELY DESTROYED
// HEAP:  ❌ COMPLETELY DESTROYED
// DISK:  ✅ Survives (files, databases, etc.)\

// So… if both disappear, why do we even have two kinds?
// It’s all about how memory is managed while the program is running.

// 1. Stack — neat, fast, temporary desk
// Managed automatically (push when you call a function, pop when it ends).
// Stores small, fixed-size data (integers, pointers, short arrays).
// Each thread has its own stack.
// Extremely fast because adding/removing is just moving a pointer.

// Example:

fn main() {
    let x = 42; // stored on stack
} // x is removed here automatically

// Key traits:
// Lifetime: Only exists until the function/block ends.
// Size: Fixed at compile time (usually a few MB).
// Speed: Lightning fast.
// Scope-bound: Disappears when it goes out of scope.

// 2. Heap — big, shared storage room
// Managed manually (you allocate space at runtime, and free it when done — Rust does this automatically via ownership).
// Stores big or dynamically sized data (e.g., Vec, String, HashMap).
// Can be shared between threads.

// Example:

fn main() {
    let v = vec![1, 2, 3]; // vector data is on heap
} // memory is freed here by Rust automatically

// Key traits:
// Lifetime: Can live beyond the function that created it (if ownership is moved).
// Size: Much larger than stack (limited by system RAM).
// Speed: Slower — you have to find a free spot in memory.
// Sharing: Multiple parts of code can hold pointers to the same heap data.
