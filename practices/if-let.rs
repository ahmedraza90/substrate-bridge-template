// flow of if let statements in Rust.
if let Some(value) = some_option {
    // Runs nested code only if some_option is Some(value)
    println!("Found: {}", value);
}
if let Ok(value) = some_result {
    // Runs nested code only if some_result is Ok(value)
    println!("Success: {}", value);
}
if let Err(error) = some_result {
    // Runs nested code only if some_result is Err(error)
    println!("Error: {}", error);
}