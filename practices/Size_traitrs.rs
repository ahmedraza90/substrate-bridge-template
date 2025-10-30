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


//INSTANCE METHOD VS STATIC METHOD

Case 1 — instance method
trait Example {
    fn hello(&self); // instance method
}


This can be called like:

let obj = MyType;
obj.hello(); // ✅ instance method


Because &self means: “I need an instance of this type.”
when we call it like this the first argument is automatically self which is needed in this case

But not like:
MyType::hello(); // ❌ invalid


🔹 Case 2 — static method (no self)
// In Rust, a static method is one that does not take &self, &mut self, or self as its first argument.
trait Example {
    fn hello();
}

You must call it like:
MyType::static_method();

But not like:
let m = MyType;
m.static_method(); // ❌ invalid because first argument is self by default here but we are not taking self in arguments so there would be problem.


This method doesn’t take self, so it doesn’t belong to any object —
it belongs to the type itself.





struct DateTime<Tz> {
    time: String,
    tz: Tz,
}

struct Utc;

trait TimeZone {
    fn now() -> DateTime<Self>
    where
        Self: Sized; // ensures we call on type, not instance
}

impl TimeZone for Utc {
    fn now() -> DateTime<Self> {
        DateTime {
            time: "2025-10-16T12:00:00Z".to_string(),
            tz: Utc,
        }
    }
}

fn main() {
    let now = Utc::now(); // ✅ works
    println!("{}", now.time);

    let utc = Utc;
    // utc.now(); // ❌ does NOT work because now() is static
}








