MAP:
// map works on Option and Result. It is used to access the value wrapped inside Option or Result to update this value. Simple!!
// If it’s Some(d) or Ok(d), run the closure. If it’s None or Err(), do nothing
// after an operation on value it wrap the value again into Some.
let result = Ok(5);
let doubled = result.map(|x| x * 2);  // Ok(10)

let error = Err("failed");
let doubled = error.map(|x| x * 2);   // Still Err("failed") - unchanged!

let some = Some(5)
let doubled = some.map(|x| x * 2)

MAP_ERR:
map_err (for Result<T, E> only):

// Works on the error case.
// Transforms the error inside Err(error).
// If Ok, it does nothing and passes through unchanged.


FILTER:
let some = Some(5)
let is_empty = filter.some(|x| !x.is_empty())

// filter works on Option like this:
// If it’s Some(value), check the condition.
// If condition is true → keep Some(value).
// If condition is false → turn it into None.
// If it’s already None, it stays None.

tasks.iter().filter(|t| t.completed)
// Walks through many items (array/vector).Skips items that don’t match condition.
// Only “yields” items that match.
// Then something like .count(), .collect(), etc. consumes them.

REF:
ref:
// Borrow the value inside Some instead of moving it.
// This lets you use description as a reference (&String) without taking ownership.
// It avoids copying or moving the actual string, so you can safely use it (e.g., call .trim()).
if let Some(ref s) = name {
    println!("{}", s); // &String
}

AS_REF:
🧩 2. The .as_ref() method (method-level borrowing)
// Instead of using ref in pattern matching, you can borrow the inner value using as_ref() method:
if let Some(s) = name.as_ref() {
    println!("{}", s); // &String
}

// CONFUSION
// There’s no semantic difference between using ref and .as_ref() — they both borrow the inner value without moving it.
🧩 2️⃣ Difference — ✳️ Pattern-level vs Method-level
Aspect	    ref	                                            .as_ref()
Type	    Pattern keyword	                                Method call
Where       used	Inside a match / if let / let pattern	Anywhere (since it’s just a method)
Works       on	Any pattern that binds	                    Any type that implements AsRef (like Option, Result, etc.)
Readability	Feels lower-level	                            Feels more idiomatic and consistent


AS_DEREF:
🧩 3. The .as_deref() method (borrow + deref)
// .as_deref() is like .as_ref() but it also automatically applies Deref — turning an Option<&String> into Option<&str>.
// This is especially nice when you only need to read data as a str, not a String.
// Example:
if let Some(description) = self.description.as_deref() {
    println!("{}", description.trim());
}
✅ Here:
// .as_deref() converts Option<String> → Option<&str>
// No need to manually write .as_ref().map(|s| s.as_str())
// 📘 Type of description: &str
println!("=== WHAT as_deref() ACTUALLY DOES ===\n");
    
    println!("as_deref() is available on:");
    println!("✅ Result<T, E> where T implements Deref");
    println!("✅ Option<T> where T implements Deref");
    println!();
    println!("Common T types that implement Deref:");
    println!("• String → &str");
    println!("• Vec<T> → &[T]");
    println!("• Box<T> → &T");
    println!("• PathBuf → &Path");
    println!("• OsString → &OsStr");


    COUNT:
    COLLECT:
    ITER:
    INTO_ITER:
    wrap(),
    unwrap();
    unwrap_or_else();