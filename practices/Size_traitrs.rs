// What is Sized?
// Sized is a special trait in Rust that means "this type has a known size at compile time."

trait SubstrateCli {
    fn from_args() -> Self; // Error! Can't return unsized type
}

// You can't return something if you don't know its size! Rust needs to know how much space to allocate on the stack.'

// In Practice
// For your use case with Cli:
impl SubstrateCli for Cli {
    // Cli is a struct with known fields = Sized ✓
}

// This would be impossible:
impl SubstrateCli for str {
    // str is NOT Sized ✗
}

// Common Pattern
// You'll often see this pattern in Rust:
trait MyTrait: Sized {
    fn create() -> Self; // Needs Sized
    fn consume(self); // Needs Sized
    fn get_box() -> Box<Self>; // Needs Sized
}
// Basically, whenever a trait has methods that:

// Return Self
// Take self by value (not &self)
// Create containers of Self

// It needs the Sized bound!



🔹 Rule #1: By Default, Traits Assume Self: Sized

When you write:

trait MyTrait {
    fn create() -> Self;
}


// Rust automatically interprets this as:

trait MyTrait: Sized {
    fn create() -> Self;
}


// So you don’t need to write : Sized explicitly in most cases — it’s already there behind the scenes.

// 🔹 Rule #2: When Do We Explicitly Write : Sized?
// You only need to write it explicitly when:
// You want to remind the reader/compiler that your trait requires Self to be sized. (Clarity)
// You’re adding other trait bounds and want to include Sized with them, e.g.:

trait MyTrait: Clone + Sized {
    fn create() -> Self;
}

// Here you’re saying:
// “This trait requires both Clone and Sized.”
// If you only had Clone, Rust would NOT assume Sized anymore — so writing it is necessary in this case.

// 🔹 Rule #3: When Do We Remove Sized (opt-out)?
// If you want your trait to work with unsized types (str, [T], dyn Trait), you must opt out:

trait MyTrait: ?Sized {
    fn do_something(&self); // takes &self, so no size needed
}


// Now this trait can be implemented for unsized types too.