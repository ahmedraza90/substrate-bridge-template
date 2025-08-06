//TRAIT BOUND

trait StaticHello {
    fn static_hello();
}

struct MyType;

impl StaticHello for MyType {
    fn static_hello() {
        println!("Hi, I am a static method!");
    }
}

fn use_it<T: StaticHello>() {
    T::static_hello();
}

fn main() {
    use_it::<MyType>();
}

// TURBO FISH

struct MyType;

impl MyType {
    fn static_hello() {
        println!("Hi, I am a static method!");
    }
}

fn use_it<T>() {
    T::static_hello();
}

fn main() {
    use_it::<MyType>(); // 👈 Turbo Fish used here
}

// Concept	                Purpose	When?	                                                                                     Runtime or Compile-Time?
// Trait Bound (T: Trait)	Restrict what kind of types are allowed	Inside function or where clause	                                ✅ Compile-time
// Turbo Fish (::<T>)	    Tell the compiler what type to use	When calling generic functions or types	                            ✅ Compile-time
// Runtime Types	        Not related to Turbo Fish or generics	Happens dynamically (e.g., trait objects like Box<dyn Trait>)	✅ Runtime

//✔️ So yes — trait bounds bind the generic type to a trait when defining a function.
// ✔️ Turbo Fish syntax is used to specify the type when calling a generic function.
