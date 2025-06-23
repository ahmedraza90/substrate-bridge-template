fn from_args() -> Self 
where 
    Self: Parser + Sized,
{
    <Self as SubstrateCli>::from_iter(&mut std::env::args_os())
}


Let me show you why this syntax exists with a simpler example:
rust// Imagine you have two traits with the same method name:
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