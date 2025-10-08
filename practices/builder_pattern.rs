// Pretend this is like "Layer"
trait AddOn {
    fn apply(&self, item: &mut String);
}

// Each topping is a layer
struct Patty;
impl AddOn for Patty {
    fn apply(&self, item: &mut String) {
        item.push_str(" + patty");
    }
}

struct Cheese;
impl AddOn for Cheese {
    fn apply(&self, item: &mut String) {
        item.push_str(" + cheese");
    }
}

struct Sauce;
impl AddOn for Sauce {
    fn apply(&self, item: &mut String) {
        item.push_str(" + sauce");
    }
}

// -----------------------------

// A builder to stack layers
struct BurgerBuilder {
    steps: Vec<Box<dyn AddOn>>,
}

impl BurgerBuilder {
    fn new() -> Self {
        Self { steps: vec![] }
    }

    fn layer<T: AddOn + 'static>(mut self, topping: T) -> Self {
        self.steps.push(Box::new(topping));
        self
    }

    fn build(self) -> String {
        let mut burger = "bun".to_string(); // start with bun
        for step in self.steps {
            step.apply(&mut burger);
        }
        burger
    }
}

// -----------------------------

fn main() {
    type BurgerReturnType = BurgerBuilder;

    fn build_named() -> BurgerReturnType {
        BurgerBuilder::new().layer(Patty).layer(Cheese).layer(Sauce)
    }

    // or
    // it means you do not know to know the exact type the only thing you need to
    // know is that it implements Addon
    fn make_burger() -> impl AddOn {
        BurgerBuilder::new().layer(Patty).layer(Cheese).layer(Sauce)
    }
}
