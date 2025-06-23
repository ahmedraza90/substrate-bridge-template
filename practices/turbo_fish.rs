// cmd.run::<Cli, Runtime>() turbofish syntax..
// Whenever we want to call the function of any specific type inside some function, we use turbofish to bring the type inside 
// the scope
struct MyType;

impl MyType {
    fn static_hello() {
        println!("Hi, I am a static method!");
    }
}

fn use_it<T>() {
    T::static_hello(); // Calling a method from the type itself
}

// Define a generic command struct
pub struct Command;

impl Command {
    // The `run` function that executes the command
    pub fn run<Cli, Runtime>(&self) -> Result<(), Box<dyn Error>>
    where
        Cli: SubstrateCli, // The CLI struct must implement SubstrateCli
        Runtime: 'static,  // The runtime must have a static lifetime
    {
        // Step 1: Parse the CLI arguments and create a configuration
        let cli_config = Cli::create_configuration()?;
        println!("CLI Configuration created: {:?}", cli_config);

        // Step 2: Initialize the node services
        let (task_manager, client) = Self::initialize_node::<Runtime>(cli_config)?;

        // Step 3: Execute the command logic
        println!("Running the command logic...");
        // Example: Perform some action based on the command
        // (e.g., build a chain spec, validate a block, etc.)

        // Step 4: Clean up and exit
        println!("Command executed successfully.");
        drop(task_manager); // Ensure the task manager is cleaned up
        Ok(())
    }

    // Helper function to initialize the node
    fn initialize_node<Runtime>(
        config: Configuration,
    ) -> Result<(TaskManager, String), Box<dyn Error>> {
        // Example: Create a TaskManager and a mock client
        let task_manager = TaskManager::new(config.task_executor.clone(), None)?;
        let client = "MockClient".to_string(); // Replace with actual client initialization
        Ok((task_manager, client))
    }
}
cmd.run::<Cli, Runtime>()

// 👀 First: What is parse()?
let num = "42".parse();
// This is a method that tries to convert a string like "42" into a number.

// But Rust doesn’t know what type you want. Do you want a u32? f64? i64?

// So you need to tell it.

// ✨ Two Ways to Tell Rust the Type
// ✅ 1. Use a variable type:
let num: u32 = "42".parse().unwrap();
// You’re telling Rust:

// “I want to parse this into a u32.”

// ✅ 2. Use turbofish syntax:
let num = "42".parse::<u32>().unwrap();
// Here you tell Rust directly:

// “Parse this string into a u32.”



