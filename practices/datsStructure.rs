// 1. What you already know: HashMap

// std::collections::HashMap<K, V> is the standard Rust hash map.
// It is not thread-safe by default.
// → If multiple threads try to read/write at the same time, you need to wrap it in a Mutex or RwLock.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let map = Arc::new(Mutex::new(HashMap::new()));

    let handles: Vec<_> = (0..10)
        .map(|i| {
            let map = map.clone();
            thread::spawn(move || {
                let mut lock = map.lock().unwrap();
                lock.insert(i, i * 10);
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    println!("{:?}", map.lock().unwrap());
}

// 2. What DashMap gives you

//     DashMap is like HashMap but designed for concurrent/multi-threaded environments.
//     dashmap is a concurrent HashMap for Rust.
//     Multiple threads can read/write simultaneously.
//     Internally, it uses sharding (splits the map into buckets, each with its own lock).
//     That means one thread writing to key "A" doesn’t block another thread reading/writing to key "B".
//     So it’s like a HashMap + RwLock, but more optimized for concurrent access.

// dashmapp === Arc<Mutex<HashMap>> ???

// 🔒 Arc<Mutex<HashMap<K, V>>>

//     Single big lock around the whole map.
//     Any thread that wants to read or write must acquire the lock.
//     That means:
//         If one thread is inserting into the map, all other threads are blocked (even readers).
//         Reads and writes cannot happen in parallel.

// ⚡ DashMap<K, V>

//     Sharded locking: the map is split internally into many small segments (buckets).
//     Each bucket has its own lock.
//     When you access a key, DashMap only locks the bucket that holds that key.
//     That means:
//         Thread A can safely insert into "apple" bucket.
//         Thread B can simultaneously read "banana" bucket.
//         They don’t block each other unless they hit the same shard.

// what is shard and what is meant by same shard:

// 🪣 First, think of a HashMap
//     A normal HashMap works like this:
//     You give it a key (like "apple").
//     It runs the key through a hash function.
//     The hash decides which bucket (storage slot) to put the value in.
//     (That’s how it finds things quickly.)
// So a HashMap is basically a bunch of buckets, each bucket holding some keys.

// 🧩 Now, what is a shard?
//     In DashMap, those buckets are grouped into shards.
//     A shard = a group of buckets + its own lock.
//     Instead of locking the whole map, DashMap only locks the shard that contains your key.

// 🍎 Example
//     Say you have a DashMap with 4 shards (in reality it’s usually 32 or more).
//         "apple" → hash → shard #1
//         "banana" → hash → shard #2
//         "carrot" → hash → shard #1
//     Now:
//         If Thread A inserts "apple", it locks shard #1.
//         If Thread B inserts "banana", it locks shard #2 → ✅ no conflict, both run at the same time.
//         If Thread C inserts "carrot", it also needs shard #1 → 🚫 must wait for Thread A to finish.
