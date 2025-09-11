let result = Ok(5);
let doubled = result.map(|x| x * 2);  // Ok(10)

let error = Err("failed");
let doubled = error.map(|x| x * 2);   // Still Err("failed") - unchanged!

let some = Some(5)
let doubled = some.map(|x| x * 2)
// map works on Option and Result. It is used to access the value wrapped inside Option or Result to update this value. Simple!!
// If it’s Some(d) or Ok(d), run the closure. If it’s None or Err(), do nothing


let result = Ok(5);

let updated = result.map(|x| x * 10);
// If result was Ok(5) → you access the 5, multiply by 10, get Ok(50)

// If result was Err("something") → the closure never runs, and you just get back Err("something")


let some = Some(5)
let is_empty = filter.some(|x| !x.is_empty())
// filter

// filter works on Option like this:
// If it’s Some(value), check the condition.
// If condition is true → keep Some(value).
// If condition is false → turn it into None.
// If it’s already None, it stays None.


// Iterator filter (what we’ve been talking about)

// tasks.iter().filter(|t| t.completed)
// Walks through many items (array/vector).Skips items that don’t match condition.
// Only “yields” items that match.
// Then something like .count(), .collect(), etc. consumes them.