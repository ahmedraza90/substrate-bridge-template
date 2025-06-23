// Define the Sandwich struct
struct Sandwich {
    bread: String,
    filling: String,
    sauce: String,
}

// Define the builder for Sandwich
struct SandwichBuilder {
    bread: String,
    filling: String,
    sauce: String,
}

impl SandwichBuilder {
    // Start with default sandwich
    fn new() -> Self {
        Self {
            bread: "White".to_string(),
            filling: "Cheese".to_string(),
            sauce: "None".to_string(),
        }
    }

    // Set bread type
    fn bread(mut self, bread: &str) -> Self {
        self.bread = bread.to_string();
        self
    }

    // Set filling
    fn filling(mut self, filling: &str) -> Self {
        self.filling = filling.to_string();
        self
    }

    // Set sauce
    fn sauce(mut self, sauce: &str) -> Self {
        self.sauce = sauce.to_string();
        self
    }

    // Build the final Sandwich
    fn build(self) -> Sandwich {
        Sandwich {
            bread: self.bread,
            filling: self.filling,
            sauce: self.sauce,
        }
    }
}

fn main() {
    // Use the builder pattern to make a sandwich
    let my_sandwich = SandwichBuilder::new()
        .bread("Whole Wheat")
        .filling("Ham")
        .sauce("Mustard")
        .build();

    println!(
        "Sandwich: {} bread, {} filling, {} sauce",
        my_sandwich.bread, my_sandwich.filling, my_sandwich.sauce
    );
}
