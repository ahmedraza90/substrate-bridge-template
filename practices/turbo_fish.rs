// ✅ What Turbo Fish really is
// Turbo Fish (::<...>) is just a syntax to explicitly tell the compiler which generic type parameter to
//  use.

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

// Trait bound (T: Trait) = restricting what kinds of types are allowed.

// 



// Example: collect()
fn main() {
    let nums = vec![1, 2, 3];

    // Collect into something...
    let v = nums.iter().map(|x| x * 2).collect::<Vec<i32>>();

    println!("{:?}", v);
}


// ✅ Works fine:

    // .map(|x| x * 2) produces an iterator.
    // .collect() can turn an iterator into many things (Vec, HashSet, String, etc.).
    // But Rust needs to know which collection.
    // Turbo Fish ::<Vec<i32>> tells the compiler explicitly.

// Alternative: let-binding with type annotation

    // Instead of Turbo Fish, you can also annotate the variable:

    let v: Vec<i32> = nums.iter().map(|x| x * 2).collect();

    // Same effect ✅, just two styles:
        // let v: Vec<i32> = ...collect();
        // let v = ...collect::<Vec<i32>>();

// Another fun case: parse()
fn main() {
    let n = "42".parse::<i32>().unwrap();  // Turbo Fish
    println!("{}", n + 1);
}


// .parse() tries to parse into any type that implements FromStr.
// Without Turbo Fish:
let n = "42".parse().unwrap(); // ❌ Error: which type? i32? f64? u8?


// Generic Functions with Multiple Type Parameters
fn parse_pair<T, U>() -> (T, U) 
where 
    T: std::str::FromStr,
    U: std::str::FromStr,
    T::Err: std::fmt::Debug,
    U::Err: std::fmt::Debug,
{
    let s1 = "42";
    let s2 = "3.14";
    (s1.parse().unwrap(), s2.parse().unwrap())
}

fn main() {
    // ❌ COMPILE ERROR: Cannot infer types
    // let result = parse_pair();
    
    // ✅ TURBO FISH REQUIRED
    let result = parse_pair::<i32, f64>();
    println!("{:?}", result); // (42, 3.14)
}

// Return Type Disambiguation
use std::collections::HashMap;

fn get_container<T>() -> T 
where 
    T: Default 
{
    T::default()
}

fn main() {
    // ❌ COMPILE ERROR: Cannot infer type
    // let container = get_container();
    
    // ✅ TURBO FISH REQUIRED  
    let map = get_container::<HashMap<String, i32>>();
    let vec = get_container::<Vec<i32>>();
    
    println!("Map: {:?}, Vec: {:?}", map, vec);
}

// 4. Method Calls on Generic Types
struct Container<T> {
    items: Vec<T>,
}

impl<T> Container<T> {
    fn new() -> Self {
        Self { items: Vec::new() }
    }
    
    fn add(&mut self, item: T) {
        self.items.push(item);
    }
}

fn main() {
    // ❌ COMPILE ERROR: Cannot infer T
    // let mut container = Container::new();
    
    // ✅ TURBO FISH REQUIRED
    let mut container = Container::<String>::new();
    container.add("hello".to_string());
    
    println!("{:?}", container.items);
}



// Noted:
    // The underscore means "infer the element type, but I'm specifying Vec"
    let numbers = (0..5).collect::<Vec<_>>();
    //                               ^^^
    //                               |
    //    "Make a Vec, but infer what goes inside"
    
    // This is equivalent to:
    let numbers = (0..5).collect::<Vec<i32>>();
    
    // Both create Vec<i32>, but <Vec<_>> lets Rust figure out the i32 part