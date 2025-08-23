match function_that_might_fail() {
    Ok(success_value) => {
        // Function succeeded
        // Do something with success_value
    }
    Err(error_value) => {
        // Function failed  
        // Handle the error_value
    }
}

// Why use this pattern?
// Because many Rust functions return Result<T, E> which means:

// Ok(T) = Success with value T
// Err(E) = Failure with error E
// You must handle both cases, and match forces you to do this safely.

// This is Rust's way of making sure you never ignore errors!