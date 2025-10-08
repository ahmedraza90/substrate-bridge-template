// Normally in generic functions, we do this:

fn print_value<T>(val: T) {
    println!("{:?}", val);
}
// Here we use T in the input: val: T.

// But in , example, we don’t need to use a real value of type T.
// We only want to read the type name.

fn create_message<T>(name: &str) -> String {
    format!(
        "Generated a message for: {} using type: {}",
        name,
        std::any::type_name::<T>()
    )
}

fn main() {
    // Explicitly pass the generic type using ::<>
    let msg = create_message::<u32>("Alice");

    println!("{}", msg);
}

// Generated a message for: Alice using type: u32


pub fn update_task<F>(&self, id: &Uuid, updater: F) -> bool 
    where
        F: FnOnce(&mut Task),


// 1. Functions and closures are values in Rust

// In Rust, a closure (or even a function) is just another value you can pass around.
// But for that to work, Rust needs to give them a "type".

// That’s where traits like Fn, FnMut, and FnOnce come in.
// They are traits that describe how something can be called like a function.

// a) Fn
//     Can be called many times.
//     Captures value by reference (&T).

//         let name = "Alice".to_string();
//         let f = || println!("Hello {}", name); // captures `&name`
//         f(); f(); // can call multiple times

// b) FnMut
//     Can be called many times, but mutates captured variables.
//     Captures by mutable reference (&mut T).

//         let mut counter = 0;
//         let mut f = || { counter += 1; println!("{}", counter); };
//         f(); f(); // works, updates counter

// c) FnOnce
//     Can only be called once.
//     Captures by value (T), so it moves ownership. ( its not about input paramtre its about capturing value from outside)

//         let name = "Alice".to_string();
//         let f = || name; // captures and moves `name`
//         println!("{}", f()); // ok
//         // f(); // ERROR: cannot call again, name was moved


// FLEXIBILITY: ADDING EXTRA LAYER OF GENERIC
// 🔹 Two ways to design
// 1. Bind trait directly in the struct
// pub struct FarmAuditRecord<T: Config> {
//     pub updated_by: T::AccountId,
//     pub updated_at: <T::TimeProvider as UnixTime>::Moment,
// }


// Here the struct itself is tied to Config.
// Any time you use FarmAuditRecord, you must bring in a T: Config.
// less flexible, but simpler.

// 2. Keep struct generic, bind trait later in a type alias
pub struct FarmAuditRecord<AccountId, Moment> {
    pub updated_by: AccountId,
    pub updated_at: Moment,
}

pub type FarmerAuditHistory<T: Config> = StorageDoubleMap<
    _,
    Blake2_128Concat, u64,
    Blake2_128Concat, u64,
    FarmAuditRecord<T::AccountId, <T::TimeProvider as UnixTime>::Moment>,
>;

// Struct is completely independent of Substrate’s Config.
// You can reuse FarmAuditRecord in:
// Unit tests with fake AccountId = u8, Moment = u64.
// Different pallets with different runtime types.
// Only when you define FarmerAuditHistory do you bind it to Substrate’s Config.

// 🔹 Your summary in my words
// Yes: we gain flexibility by keeping the struct generic and only applying trait bounds in the type alias (or wherever we “instantiate” it).
// Yes: you can bind it differently in different contexts — e.g., one alias for pallet storage (using T::AccountId), another alias for testing (using u8).