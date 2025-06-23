macro_rules! construct_runtime {
// macro_rules! construct_runtime defines the macro.
    (
        pub enum $runtime_name:ident where
            Block = $block:ty,
            NodeBlock = $node_block:ty,
            UncheckedExtrinsic = $unchecked_extrinsic:ty,
        {
            $($module_name:ident : $module_path:ident),* $(,)?
            // Purpose:
            // The $(,)? allows the macro to accept an optional trailing comma after the last item in the list. This is useful because, in Rust, it is common to allow trailing commas in lists for better readability and easier editing.
            // The * after the $() repetition operator means "zero or more repetitions."
            // , Indicates that each pair (module_name: module_path) is separated by a comma.
            // (): This is a repetition operator in Rust macros.
            // The :ident specifies that this placeholder must match an identifier.
        }
    ) => {
        // Define the runtime enum
        pub enum $runtime_name {
            $(
                $module_name,
            )*
        }

        // Define the runtime struct with associated types
        pub struct Runtime {
            pub block: $block,
            pub node_block: $node_block,
            pub unchecked_extrinsic: $unchecked_extrinsic,
        }

        impl Runtime {
            pub fn new() -> Self {
                Runtime {
                    block: Default::default(),
                    node_block: Default::default(),
                    unchecked_extrinsic: Default::default(),
                }
            }

            // Example function to simulate calling a module
            pub fn call_module(&self, module: $runtime_name) {
                match module {
                    $(
                        $runtime_name::$module_name => {
                            println!("Calling module: {}", stringify!($module_name));
                            // stringify!: Converts the module name into a string for printing.
                        }
                    )*
                }
            }
        }
    };
}

construct_runtime!(
    pub enum TestRuntime where
        Block = String,
        NodeBlock = String,
        UncheckedExtrinsic = String,
    {
        System: frame_system,
        MyPallet: my_pallet,
    }
);

// Define a macro to create constant types
macro_rules! parameter_types {
    ($($name:ident: $type:ty = $value:expr;)*) => {
        $(
            pub struct $name;

            impl $name {
                pub const VALUE: $type = $value;
            }
        )*
    };
}

// Use the macro to define constants
parameter_types! {
    BlockHashCount: u64 = 250;
    MaxBlockWeight: u32 = 1024;
}

// this generates
pub struct BlockHashCount;
impl BlockHashCount {
    pub const VALUE: u64 = 250;
}

pub struct MaxBlockWeight;
impl MaxBlockWeight {
    pub const VALUE: u32 = 1024;
}

fn main() {
    // Create a new runtime instance
    let runtime = Runtime::new();

    // Simulate calling modules
    runtime.call_module(TestRuntime::System);
    runtime.call_module(TestRuntime::MyPallet);
}
