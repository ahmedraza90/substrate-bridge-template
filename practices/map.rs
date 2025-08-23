let result = Ok(5);
let doubled = result.map(|x| x * 2);  // Ok(10)

let error = Err("failed");
let doubled = error.map(|x| x * 2);   // Still Err("failed") - unchanged!


// map is used to access the value wrapped inside Ok to update this value. Simple!!


let result = Ok(5);

let updated = result.map(|x| x * 10);
// If result was Ok(5) → you access the 5, multiply by 10, get Ok(50)

// If result was Err("something") → the closure never runs, and you just get back Err("something")