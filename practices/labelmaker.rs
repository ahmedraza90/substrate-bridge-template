// Like having a medicine bottle labeled "For Dogs" 
// even though the bottle itself doesn't contain a dog
struct MedicineBottle<Animal> {
    pills: Vec<Pill>,
    _for_animal: PhantomData<Animal>,  // Just a label/marker
}

// Now you can have:
MedicineBottle<Dog>    // Pills for dogs
MedicineBottle<Cat>    // Pills for cats (different type!)


// what is the purpose of just label marker. 

pub struct BlocksClient<T, Client> {
    client: Client,
    _marker: PhantomData<T>,  // This T affects the METHODS available
}

impl<Client> BlocksClient<Config32bit, Client> {
    pub fn get_hash(&self) -> Hash32 { /* ... */ }
}

impl<Client> BlocksClient<Config64bit, Client> {
    pub fn get_hash(&self) -> Hash64 { /* ... */ }  // Different return type!
}

// The Point:
// The T parameter changes what methods are available and what types they return, even though T itself isn't stored as data.
