// Define an enum
enum Animal {
    Dog,
    Cat,
}

// Define a struct associated with the Animal enum
struct AnimalConfig {
    name: String,
    age: u8,
}

// Define a type alias for convenience and clarity
type AnimalGenesisConfig = AnimalConfig;

impl Animal {
    // Associated function to create a default AnimalConfig
    fn default_config() -> AnimalGenesisConfig {
        AnimalGenesisConfig {
            name: "Unknown".to_string(),
            age: 0,
        }
    }

    // Associated function to create a custom AnimalConfig
    fn custom_config(name: &str, age: u8) -> AnimalGenesisConfig {
        AnimalGenesisConfig {
            name: name.to_string(),
            age,
        }
    }
}

// as far as i understand so the most strong advantage i see is that..
// ✅ 2. Flexibility for future changes
// Let's say later you want to make AnimalGenesisConfig a completely different struct.

// If you’ve already used the alias everywhere, you only need to change:
// type AnimalGenesisConfig = NewAnimalGenesisStruct;
// No need to update all usages across your codebase.

// second advantage could be sometimes about get rid of complex names just assigning new or simple name to already define types.
