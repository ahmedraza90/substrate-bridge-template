// Normally in generic functions, we do this:

fn print_value<T>(val: T) {
    println!("{:?}", val);
}
// Here we use T in the input: val: T.

// But in , example, we don’t need to use a real value of type T.
// We only want to read the type name.

fn create_message<T>(name: &str) -> String {
    format!(
        "Generated a message for: {} using type: {}",
        name,
        std::any::type_name::<T>()
    )
}

fn main() {
    // Explicitly pass the generic type using ::<>
    let msg = create_message::<u32>("Alice");

    println!("{}", msg);
}

// Generated a message for: Alice using type: u32


pub fn update_task<F>(&self, id: &Uuid, updater: F) -> bool 
    where
        F: FnOnce(&mut Task),


// 1. Functions and closures are values in Rust

// In Rust, a closure (or even a function) is just another value you can pass around.
// But for that to work, Rust needs to give them a "type".

// That’s where traits like Fn, FnMut, and FnOnce come in.
// They are traits that describe how something can be called like a function.

a) Fn

Can be called many times.

Captures things by reference (&T).

let name = "Alice".to_string();
let f = || println!("Hello {}", name); // captures `&name`
f(); f(); // can call multiple times

b) FnMut

Can be called many times, but mutates captured variables.

Captures by mutable reference (&mut T).

let mut counter = 0;
let mut f = || { counter += 1; println!("{}", counter); };
f(); f(); // works, updates counter

c) FnOnce

Can only be called once.

Captures by value (T), so it moves ownership.

let name = "Alice".to_string();
let f = || name; // captures and moves `name`
println!("{}", f()); // ok
// f(); // ERROR: cannot call again, name was moved