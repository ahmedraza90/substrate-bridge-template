// Arc<T> means:
// “Allow multiple threads to have a reference to the same data.”

// It uses atomic reference counting:

// Every time you clone the Arc, it increases a counter.

// When the last thread drops it, the data is freed.

// 🧵 Thread + Arc in Action (Simple Rust Example):
use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(vec![1, 2, 3]);

    for _ in 0..3 {
        let data_cloned = Arc::clone(&data);
        thread::spawn(move || {
            println!("{:?}", data_cloned);
        });
    }
}
// One vector (vec![1,2,3])

// Cloned into 3 threads safely using Arc

// Each thread prints it without crashing

// using Arc can we only read data or write also ?

// ✅ Short Answer:
// With just Arc<T>, you can only read shared data across threads.
// If you want to write (mutate) the data, you must wrap it in a Mutex<T> as well:

// Arc<Mutex<T>>  → allows safe shared **read + write**
// Arc<T>         → allows shared **read only**

// 🔍 Why can't Arc<T> alone write?
// Arc<T> gives shared ownership but not shared mutability.

// Rust won't allow multiple threads to mutate the same data unless it's protected — to avoid race conditions (two threads writing at the same time).

// ✅ To Read + Write: Use Arc<Mutex<T>>
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(0)); // Shared AND mutable

    let mut handles = vec![];

    for _ in 0..5 {
        let data_cloned = Arc::clone(&data);

        let handle = thread::spawn(move || {
            let mut locked = data_cloned.lock().unwrap();
            *locked += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final result: {}", *data.lock().unwrap()); // Should print 5
}

// 🧠 What happens:
// Arc lets multiple threads share the data.

// Mutex makes sure only one thread at a time can access the value inside (to mutate it).

// lock() gives you access to the data, blocking other threads until you're done.

// 🧠 Analogy:
// Arc<T> → Everyone can see the whiteboard.

// Arc<Mutex<T>> → Everyone must hold the marker (mutex) to write. Only one can write at a time.

// Arc
// Purpose: Share ownership of some data on the heap between threads.

// Limitation: Does nothing to control when or how the data is accessed.

// Without a lock, multiple threads could read/write at the same time → data race.

// Mutex
// Purpose: Make sure only one thread is inside the critical section (reading or writing) at a time.

// This protects against simultaneous writes and reads during writes.

// ⚠️ Nuance:
// Arc is not always incomplete without Mutex.

// If the shared data is immutable (never changes after creation), you can just use Arc<T> without Mutex because multiple reads at the same time are safe.

// Example: Arc<Vec<String>> that you only read from in multiple threads — no Mutex needed.

// But if the data is mutable (can be changed by threads), then you need Arc + Mutex (or some other lock like RwLock).
