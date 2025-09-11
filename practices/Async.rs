// 1. Why async?
// We mark a function async when we want it to be able to pause and resume instead of blocking the thread.

// async fn fetch_data() {
//     // can use .await inside here
// }
// An async function returns a future — basically, a promise to produce a value later.

// 2. Why await?
// We use .await when the task might take time (I/O, network, file read, etc.) and we don’t want to freeze the thread.

// let result = fetch_data().await;
// 3. The twist — no freezing the program

// When you hit .await:
//     The current async function pauses.
//     Control goes back to the executor (like Tokio).
//     The executor runs other tasks while waiting.
//     When the awaited task finishes, your function resumes from where it left off.
//     So, .await is not like sleep() — it doesn’t block the CPU; it lets other async work happen.

// What actually happens
// When you call .await inside an async function, only that async function pauses.
// The thread is not blocked — the async executor can run other ready tasks in the meantime.
// This means that other async functions or tasks (not just “other functions in main”) can keep running.


// There are TWO Different Types of Runtimes
// 1. Regular Runtime (Always Present)
main() {
    // This runs on Rust's regular runtime
    println!("Hello");
    let file = std::fs::read_to_string("file.txt");  // Blocking I/O
    println!("Done");
}
// What this runtime does:

// Executes your code line by line
// Manages memory
// Handles system calls
// BUT: When you do I/O, everything stops and waits

// 2. Async Runtime (Optional - for async code)
#[tokio::main]  // ← Adding async runtime ON TOP of regular runtime
async fn main() {
    println!("Hello");  // Still runs on regular runtime
    let file = tokio::fs::read_to_string("file.txt").await;  // Async I/O
    println!("Done");
}
// What async runtime adds:

// Can pause and resume functions
// Can run multiple tasks concurrently
// Doesn't block on I/O


// Regular Runtime + Async Code = Confusion
main() {
    let future = fetch_data();  // This returns a Future<Output = String>
    
    // Regular runtime sees this and says:
    // "I don't know what a Future is! I can only run regular code!"
    
    println!("{:?}", future);  // This just prints the Future object, doesn't run it
}

async fn fetch_data() -> String {
    "data".to_string()
}
// Async Runtime + Async Code = Works
#[tokio::main]
async fn main() {
    let result = fetch_data().await;  // Tokio knows how to run Futures
    
    // Async runtime says:
    // "I know how to execute this Future and get the actual result!"
    
    println!("{}", result);  // Prints: "data"
}



// Sequential (Your Description)
#[tokio::main]
async fn main() {
    println!("Starting at: {:?}", std::time::Instant::now());
    
    // These run one after another
    slow_task("Task 1").await;  // Takes 2 seconds
    slow_task("Task 2").await;  // Takes 2 seconds  
    slow_task("Task 3").await;  // Takes 2 seconds
    
    println!("Total time: ~6 seconds");
}

async fn slow_task(name: &str) {
    println!("{} starting", name);
    tokio::time::sleep(Duration::from_secs(2)).await;
    println!("{} finished", name);
}
// Output:
// Starting at: ...
// Task 1 starting
// Task 1 finished      (after 2 seconds)
// Task 2 starting
// Task 2 finished      (after 4 seconds total)
// Task 3 starting  
// Task 3 finished      (after 6 seconds total)
// Total time: ~6 seconds


// Concurrent (True Concurrency)
#[tokio::main]
async fn main() {
    println!("Starting at: {:?}", std::time::Instant::now());
    
    // These all start at the same time
    let task1 = tokio::spawn(slow_task("Task 1"));
    let task2 = tokio::spawn(slow_task("Task 2"));
    let task3 = tokio::spawn(slow_task("Task 3"));
    
    // Wait for all to finish
    task1.await.unwrap();
    task2.await.unwrap();
    task3.await.unwrap();
    
    println!("Total time: ~2 seconds");
}
// Output:
// Starting at: ...
// Task 1 starting
// Task 2 starting      (immediately after Task 1)
// Task 3 starting      (immediately after Task 2)
// Task 1 finished      (after 2 seconds)
// Task 2 finished      (after 2 seconds)  
// Task 3 finished      (after 2 seconds)
// Total time: ~2 seconds


// Concurrency vs Parallelism
// Concurrency (What Tokio Does)
// One CPU core(single threaded), switching between tasks very fast
// Tokio on a single core:
// Time: 0ms   10ms   20ms   30ms   40ms   50ms
// Core: [Task1][Task2][Task3][Task1][Task2][Task1]
//       ↑ Switches between tasks very quickly


// Parallelism (True Simultaneous Execution)
// Multiple CPU cores(multi-threaded), actually running at the exact same time
// rust// True parallelism:
// Time: 0ms   10ms   20ms   30ms   40ms   50ms
// Core1: [Task1][Task1][Task1][Task1][Task1][Task1]
// Core2: [Task2][Task2][Task2][Task2][Task2][Task2]  
// Core3: [Task3][Task3][Task3][Task3][Task3][Task3]
//        ↑ All running simultaneously on different cores


// Default Tokio (Single-threaded)
#[tokio::main]
async fn main() {
    // These 3 tasks share 1 CPU core
    let task1 = tokio::spawn(async { heavy_computation().await });
    let task2 = tokio::spawn(async { heavy_computation().await });
    let task3 = tokio::spawn(async { heavy_computation().await });
    
    // Tokio rapidly switches between them on 1 core
}

// What actually happens:
// Single CPU Core:
// Time 0-10ms:  Working on Task 1
// Time 10-20ms: Working on Task 2  
// Time 20-30ms: Working on Task 3
// Time 30-40ms: Back to Task 1
// ...and so on
// When Does Tokio Become Parallel?

// Multi-threaded Tokio Runtime
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    // Now we have 4 CPU cores available
    let task1 = tokio::spawn(async { heavy_computation().await });
    let task2 = tokio::spawn(async { heavy_computation().await });
    let task3 = tokio::spawn(async { heavy_computation().await });
    
    // These might run on different cores = TRUE PARALLELISM
}



// The Key Insight: I/O vs CPU Work
// For I/O Tasks (Network, Files, etc.)
// Concurrency is often better than parallelism
async fn fetch_url(url: &str) -> String {
    // Most time is spent waiting for network response
    // CPU is mostly idle during the wait
    reqwest::get(url).await.unwrap().text().await.unwrap()
}

// Even on 1 core, these can overlap efficiently:
let task1 = tokio::spawn(fetch_url("http://api1.com"));
let task2 = tokio::spawn(fetch_url("http://api2.com"));
let task3 = tokio::spawn(fetch_url("http://api3.com"));

// Timeline:
// Task 1: [send request]----[waiting]----[receive response]
// Task 2:    [send request]----[waiting]----[receive response]  
// Task 3:       [send request]----[waiting]----[receive response]
//         ↑ All waiting periods overlap


// For CPU-Heavy Tasks
// Parallelism is better
async fn heavy_computation() -> u64 {
    // This actually uses CPU the whole time
    (0..1_000_000_000).sum()
}

// On single core: still takes 3x as long
// On multi-core: can actually run simultaneously