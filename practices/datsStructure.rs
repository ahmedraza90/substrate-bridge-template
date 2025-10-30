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

🧩 what is DashMap:
DashMap splits the map into “shards.”
A shard = a group of buckets (Each shard is a small hashmap (with its own buckets and its own lock).)
Each shard has its own lock.
When you access a key, DashMap only locks the shard that holds the bucket that holds that key. no other thread can access that shard.
Threads operating on different shards can mutate at the same time safely.

🧩 How it actually works internally

DashMap internally contains N small hash maps, typically 32 or 64 shards (depending on configuration).
Each shard is a normal HashMap protected by a lock (RwLock).

When you insert or read a key:
DashMap uses part of the key’s hash to decide which shard to use.
It then locks only that one shard, not the entire map.
So if threads are working with keys that fall into different shards, they can truly mutate in parallel.

⚙️ Visualization
DashMap
 ├── Shard 0  → RwLock<HashMap<bucket0, bucket1, ...>>
 ├── Shard 1  → RwLock<HashMap<bucket0, bucket1, ...>>
 ├── Shard 2  → RwLock<HashMap<bucket0, bucket1, ...>>
 └── ...


Each shard = one RwLock<HashMap>
Each HashMap = multiple buckets

So threads working on Shard 0 and Shard 1 can both hold locks at the same time.

CONCURRENT OR PARALLEL:
It’s concurrent by design (thread-safe).
And if your CPU has multiple cores, those threads can also run in parallel, so DashMap operations can truly happen at the exact same time across cores.


//ARC/MUTEX vs DASHAP:
Arc → allows shared ownership of data.
DashMap → allows safe concurrent mutation inside it.


A Mutex<HashMap<_, _>> has one big global lock.
Only one thread can mutate or read the map at a time.
➜ Other threads must wait.
-> we need manual locks

In contrast, DashMap uses many small locks (called shards).
The map is split into many buckets.
Each bucket has its own small lock.
Threads modifying different keys can work truly in parallel.

// Thread A modifies key 1  → shard #1 lock
// Thread B modifies key 2  → shard #7 lock
// Both happen at the same time!


DASHMAP WITHOUT ARC:
use dashmap::DashMap;
use std::thread;

fn main() {
    let map = DashMap::new();

    // Try to move map into two threads (will not compile)
    let handle1 = thread::spawn(move || {
        map.insert(1, "a");
    });

    let handle2 = thread::spawn(move || {
        map.insert(2, "b");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

DASHMAP WITH ARC:
use dashmap::DashMap;
use std::sync::Arc;
use std::thread;

fn main() {
    let map = Arc::new(DashMap::new());

    let map1 = Arc::clone(&map);
    let handle1 = thread::spawn(move || {
        map1.insert(1, "a");
    });

    let map2 = Arc::clone(&map);
    let handle2 = thread::spawn(move || {
        map2.insert(2, "b");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    // Both inserts are visible in the same DashMap instance
    for entry in map.iter() {
        println!("{:?} => {:?}", entry.key(), entry.value());
    }
}

QUESTION:
“In Rust, thread::spawn is for concurrency. Even if there are multiple cores, the work will be done in a concurrent way.”
ANSWER:
❌ Almost right, but here’s the small correction:
thread::spawn creates a real OS thread (not a simulated or async task).
That means:
If your machine has 1 core, those threads will run concurrently (taking turns).
If your machine has multiple cores, those threads can run in parallel (at the same time).

So — thread::spawn does not limit you to concurrency.
It gives you true OS threads, and the OS decides whether to schedule them concurrently or in parallel, depending on available cores.


Rust gives you both options
Rust concept	                Type of behavior	                            How it runs
thread::spawn	                Multithreading (can be concurrent or parallel)	Uses real OS threads; OS decides scheduling
async/await	                    Concurrency only	                            Single-threaded (unless you use async runtime with multiple threads)


Multithreading is not limited to concurrency.
It simply means a program has multiple threads of execution — and whether those threads run concurrently or in parallel depends entirely on the hardware (number of CPU cores) and the OS scheduler.

🧠 In summary
Concep          Meaning	                                        Controlled by
Multithreading	Program uses more than one thread	            The programmer
Concurrency	    Threads appear to run together (take turns)	    OS scheduler
Parallelism	    Threads actually run together on multiple cores	Hardware (CPU cores)


CASES:
1#
in case of multi core suppoe two threads tries to change data in parallel. in my understanding two threads can not change same data simultaneously. 
my answer is: even in multi core they can not do it because there are locks on each shard they will take case of it.

2#
there is a ssingle core and work is being done concurrently.. thread A remove the value first then thread B take turn and access that value then ??
thread B will look out for that value and it simply gets None.