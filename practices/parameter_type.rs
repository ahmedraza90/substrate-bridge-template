// 🔎 1. What Rust allows in type
// When you implement a trait, an associated type must always be another type, not a raw value.
// Example (works fine):

trait Example {
    type NumType;
}

struct MyStruct;

impl Example for MyStruct {
    type NumType = u32;   // ✅ this is a type
}

// 🔎 2. What doesn’t work
// If you try to do:

impl VendingMachineConfig for MyVendingMachine {
    type MaxItems = 50;   // ❌ ERROR
}


// Rust complains because:
// 50 is a value (u32 number), not a type.
// Associated types must be something like u32, String, or a struct — never just a literal.
// So the compiler says: “I expected a type, but you gave me a number.”


// 2. Substrate’s Way (with parameter_types! ✅)
// Substrate solves this with the parameter_types! macro.
// It generates a little struct + trait implementation that "knows" the number:

parameter_types! {
    pub const MaxItems: u32 = 50;
}


// This secretly expands into something like:

pub struct MaxItems;

impl frame_support::traits::Get<u32> for MaxItems {
    fn get() -> u32 {
        50
    }
}


// So now:
// MaxItems is a type (struct ✅)
// It also implements Get<u32> so it can return the number 50

// 3. Using It in Config
// Now you can write:

impl VendingMachineConfig for MyVendingMachine {
    type MaxItems = MaxItems;   // ✅ type, not number
}

// Consistency across all configs
// Substrate has tons of constants in pallet configs:
// BlockHashCount, ExistentialDeposit, TransactionByteFee, etc.
// Instead of everyone writing structs + impl Get manually (which is error-prone), the macro guarantees the exact same implementation everywhere.

// 📝 So the difference is:

// Your manual way works ✅ but is verbose.
// parameter_types! is a shorthand macro that auto-writes the struct + impl Get for you.
// That’s all. Think of it as a code generator for constants.