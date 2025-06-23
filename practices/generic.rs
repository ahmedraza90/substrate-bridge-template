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
