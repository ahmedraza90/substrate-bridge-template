fn from_args() -> Self 
where 
    Self: Parser + Sized,
{
    <Self as SubstrateCli>::from_iter(&mut std::env::args_os())
}


// Let me show you why this syntax exists with a simpler example:
// Imagine you have two traits with the same method name:
trait Animal {
    fn speak() {
        println!("Some animal sound");
    }
}

trait Robot {
    fn speak() {
        println!("Beep boop");
    }
}

struct runtime;
impl Animal for runtime {}
impl Robot for runtime {}  // Dog implements both!

// Now, which speak() do we call?
Dog::speak();  // ERROR! Ambiguous!

// We must specify:
<runtime as Animal>::speak();  // "Some animal sound"
<runtime as Robot>::speak();   // "Beep boop"




// ## 1. Direct Associated Types with Trait Bounds

trait MyConfig {
    type AccountId: Debug + Clone + Ord;
    type Balance: Copy + Default;
}

struct User<T: MyConfig> {
    id: T::AccountId,    // Must implement Debug, Clone, Ord
    balance: T::Balance, // Must implement Copy, Default
}

// - **Usage:** You can use `T::AccountId` and `T::Balance` directly.
// - **Why:** Rust knows these types come from your trait (`MyConfig`) and must satisfy the trait bounds you specify.

// ## 2. Associated Types from Another Trait

trait TimeTrait {
    type Moment: Copy + Ord;
}

trait MyConfig {
    type TimeProvider: TimeTrait;
}

struct Audit<T: MyConfig> {
    // This will NOT work:
    // time: T::TimeProvider::Moment, // ERROR: ambiguous

    // This is the correct way:
    time: <T::TimeProvider as TimeTrait>::Moment, // Must implement Copy, Ord
}

// - **Usage:** You must use `<T::TimeProvider as TimeTrait>::Moment`.
// - **Why:** Rust needs to know which trait provides `Moment`, and it must satisfy the trait bounds from `TimeTrait`.

// ## Key Takeaway

// - **Direct associated types**: Use `T::TypeName`, must satisfy trait bounds you specify.
// - **Trait-associated types**: Use `<Type as Trait>::AssocType`, must satisfy trait bounds from the trait.

