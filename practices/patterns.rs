#[derive_where(Clone)]
pub struct OnlineClient<T: Config> {
    inner: Arc<RwLock<Inner<T>>>,
    backend: Arc<dyn Backend<T>>,
}

impl<T: Config> OnlineClient<T> {
    /// Work with blocks.
    pub fn blocks(&self) -> BlocksClient<T, Self> {
        //calling method from another implementation of OnlineClient
        // here Self is for accessing another implementation of OnlineClient
        <Self as OfflineClientT<T>>::blocks(self)
    }

    pub fn storage(&self) -> StorageClient<T, Self> {
        <Self as OfflineClientT<T>>::storage(self)
    }

    pub fn custom_values(&self) -> CustomValuesClient<T, Self> {
        <Self as OfflineClientT<T>>::custom_values(self)
    }
}

pub trait OfflineClientT<T: Config>: Clone + Send + Sync + 'static {
    ///**** In trait methods can have logic that will be use as default if not explicitly define in implementation like here
    /// in this case.
    fn blocks(&self) -> BlocksClient<T, Self> {
        BlocksClient::new(self.clone())
    }

    fn storage(&self) -> StorageClient<T, Self> {
        StorageClient::new(self.clone())
    }

    fn constants(&self) -> ConstantsClient<T, Self> {
        ConstantsClient::new(self.clone())
    }
}

#[derive_where(Clone; Client)]
pub struct BlocksClient<T, Client> {
    // OnlineClient Type is being used here in client. so
    client: Client,
    _marker: PhantomDataSendSync<T>,
}

impl<T, Client> BlocksClient<T, Client> {
    /// Create a new [`BlocksClient`].
    pub fn new(client: Client) -> Self {
        Self {
            client,
            _marker: PhantomDataSendSync::new(),
        }
    }
}

/// Query the runtime storage.
#[derive_where(Clone; Client)]
pub struct StorageClient<T, Client> {
    client: Client,
    _marker: PhantomData<T>,
}

impl<T, Client> StorageClient<T, Client> {
    /// Create a new [`StorageClient`]
    pub fn new(client: Client) -> Self {
        Self {
            client,
            _marker: PhantomData,
        }
    }
}

/// A client for accessing constants.
#[derive_where(Clone; Client)]
pub struct ConstantsClient<T, Client> {
    client: Client,
    _marker: std::marker::PhantomData<T>,
}

impl<T, Client> ConstantsClient<T, Client> {
    /// Create a new [`ConstantsClient`].
    pub fn new(client: Client) -> Self {
        Self {
            client,
            _marker: std::marker::PhantomData,
        }
    }
}

//  Why This Pattern is Used
// ✅ Advantages:
// Separation of Concerns: OnlineClient doesn't need all block-related methods
// Organized API: Related methods are grouped together
// Type Safety: Each client knows its specific domain
// Composability: Easy to add new specialized clients
// Clean Interface: Main client stays focused, helpers handle specifics

// OnlineClient
// ├── .tx()        → TxClient       (transaction methods)
// ├── .storage()   → StorageClient  (storage methods)
// ├── .blocks()    → BlocksClient   (block methods)
// ├── .events()    → EventsClient   (event methods)
// └── .constants() → ConstantsClient (constant methods)

pub struct OnlineClient<T: Config> {
    inner: Arc<RwLock<Inner<T>>>,
    backend: Arc<dyn Backend<T>>,
}

pub struct BlocksClient<T, Client> {
    client: Client,
    _marker: PhantomDataSendSync<T>,
}

pub struct StorageClient<T, Client> {
    client: Client,
    _marker: PhantomDataSendSync<T>,
}

pub struct EventClient<T, Client> {
    client: Client,
    _marker: PhantomDataSendSync<T>,
}

// This particular style doesn’t have a single “official” GoF name, but in Rust and API design discussions, it’s usually referred to as one (or a mix) of these:

// 1. Facade + Sub-Clients (a.k.a. "Modular Client Pattern")

// Facade: OnlineClient<T> is the main entry point (the façade) — the single thing a user instantiates.
// Sub-clients: BlocksClient, StorageClient, EventsClient, etc. are specialized “sub-facades” that group related functionality.
// How it works: The main client just hands back lightweight wrappers around itself (self.clone()), and the wrappers expose methods grouped by domain.
// 👉 This is very common in Rust SDKs (like aws-sdk-rust, subxt, or database clients).

// . Type-Safe Modular Builder Pattern

// Each sub-client is generic over the main client type (Client).
// PhantomData ensures the types still carry the correct runtime marker (T: Config).
// This makes each client safe and composable — you can only call storage APIs from a StorageClient, block APIs from a BlocksClient, etc.

// delegation pattern:
// What is this pattern?
// Public API methods (like get, get_mut, value, value_mut) are defined as simple, user-facing functions.
// These methods delegate (call) to a private or internal function (often with a _ prefix, like _get, _get_mut, etc.) that does the actual work.
// The public method may do some argument checking, documentation, or setup, but the core logic is in the internal function.

pub fn value(&self) -> &V {
    self.pair().1
}

pub fn pair(&self) -> (&K, &V) {
    unsafe { (&*self.k, &*self.v) }
}

pub fn value_mut(&mut self) -> &mut V {
    self.pair_mut().1
}

pub fn pair_mut(&mut self) -> (&K, &mut V) {
    unsafe { (&*self.k, &mut *self.v) }
}

// Why use this pattern?
// Encapsulation:
// Keeps the internal logic hidden and allows changing internals without breaking the public API.

// Code reuse:
// Internal helpers can be reused by multiple public methods or by other internal code.

// Cleaner public API:
// Public methods are easy to read and document, while complex logic is separated.

// Safety and consistency:
// Public methods can enforce invariants or pre/post-conditions before/after calling the internal function.
