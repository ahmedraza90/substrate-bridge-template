// Events for PalletA
pub enum PalletAEvent {
    EventA1,
    EventA2,
}

// Events for PalletB
pub enum PalletBEvent {
    EventB1,
    EventB2,
}

// Events for PalletC
pub enum PalletCEvent {
    EventC1,
    EventC2,
}

// Events for frame_system
pub enum SystemEvent {
    BlockFinalized,
    AccountCreated,
}


// Unified runtime event type
pub enum RuntimeEvent {
    System(SystemEvent),
    PalletA(PalletAEvent),
    PalletB(PalletBEvent),
    PalletC(PalletCEvent),
}

// Manually converting events
fn handle_event(event: RuntimeEvent) {
    match event {
        RuntimeEvent::System(system_event) => match system_event {
            SystemEvent::BlockFinalized => println!("Block finalized"),
            SystemEvent::AccountCreated => println!("Account created"),
        },
        RuntimeEvent::PalletA(pallet_a_event) => match pallet_a_event {
            PalletAEvent::EventA1 => println!("PalletA EventA1"),
            PalletAEvent::EventA2 => println!("PalletA EventA2"),
        },
        RuntimeEvent::PalletB(pallet_b_event) => match pallet_b_event {
            PalletBEvent::EventB1 => println!("PalletB EventB1"),
            PalletBEvent::EventB2 => println!("PalletB EventB2"),
        },
        RuntimeEvent::PalletC(pallet_c_event) => match pallet_c_event {
            PalletCEvent::EventC1 => println!("PalletC EventC1"),
            PalletCEvent::EventC2 => println!("PalletC EventC2"),
        },
    }
}

Issues Without From

Manual Conversion:
You need to manually wrap each pallet's event into the RuntimeEvent type whenever an event is emitted.

// For example:
let event = PalletAEvent::EventA1;
let runtime_event = RuntimeEvent::PalletA(event);

Tedious Matching:
Every time you handle events, you need to write a lot of boilerplate code to match and process each event type.

Error-Prone:
Forgetting to handle a specific event type or mismatching types can lead to runtime errors or bugs.


// Using From for Automatic Conversion
// By implementing the From trait, you can automatically convert each pallet's event into the unified RuntimeEvent type

// Unified runtime event type
pub enum RuntimeEvent {
    System(SystemEvent),
    PalletA(PalletAEvent),
    PalletB(PalletBEvent),
    PalletC(PalletCEvent),
}

// Implement From for each pallet's event
impl From<SystemEvent> for RuntimeEvent {
    fn from(event: SystemEvent) -> Self {
        RuntimeEvent::System(event)
    }
}

impl From<PalletAEvent> for RuntimeEvent {
    fn from(event: PalletAEvent) -> Self {
        RuntimeEvent::PalletA(event)
    }
}

impl From<PalletBEvent> for RuntimeEvent {
    fn from(event: PalletBEvent) -> Self {
        RuntimeEvent::PalletB(event)
    }
}

impl From<PalletCEvent> for RuntimeEvent {
    fn from(event: PalletCEvent) -> Self {
        RuntimeEvent::PalletC(event)
    }
}


// That .from() is the conversion!
// It wraps your pallet's event into the big runtime-level event like this:

// Substrate auto-generates this kind of enum:
enum RuntimeEvent {
    MyPallet(Event<MyPallet>), // for your pallet
    Balances(Event<Balances>), // for another pallet
    // ...
}


// What Does Self Represent?
// Self refers to the type that is implementing the Config trait for your pallet.
// In the context of Substrate, Self is typically the runtime or mock runtime that implements the Config trait for your pallet.


// Self refers to the type that is implementing the Config trait for your pallet.
// In the context of Substrate, Self is typically the runtime or mock runtime that implements the Config trait for your pallet.