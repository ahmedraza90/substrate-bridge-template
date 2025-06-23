// What is Sized?
// Sized is a special trait in Rust that means "this type has a known size at compile time."

trait SubstrateCli {
    fn from_args() -> Self;  // Error! Can't return unsized type
}

// You can't return something if you don't know its size! Rust needs to know how much space to allocate on the stack.'

// In Practice
// For your use case with Cli:
rustimpl SubstrateCli for Cli {
    // Cli is a struct with known fields = Sized ✓
}

// This would be impossible:
impl SubstrateCli for str {
    // str is NOT Sized ✗
}


// Common Pattern
// You'll often see this pattern in Rust:
rusttrait MyTrait: Sized {
    fn create() -> Self;        // Needs Sized
    fn consume(self);           // Needs Sized
    fn get_box() -> Box<Self>;  // Needs Sized
}
// Basically, whenever a trait has methods that:

// Return Self
// Take self by value (not &self)
// Create containers of Self

// It needs the Sized bound!