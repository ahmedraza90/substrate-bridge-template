//! # Template Pallet
//!
//! A pallet with minimal functionality to help developers understand the essential components of
//! writing a FRAME pallet. It is typically used in beginner tutorials or in Substrate template
//! nodes as a starting point for creating a new pallet and **not meant to be used in production**.
//!
//! ## Overview
//!
//! This template pallet contains basic examples of:
//! - declaring a storage item that stores a single `u32` value
//! - declaring and using events
//! - declaring and using errors
//! - a dispatchable function that allows a user to set a new value to storage and emits an event
//!   upon success
//! - another dispatchable function that causes a custom error to be thrown
//!
//! Each pallet section is annotated with an attribute using the `#[pallet::...]` procedural macro.
//! This macro generates the necessary code for a pallet to be aggregated into a FRAME runtime.
//!
//! Learn more about FRAME macros [here](https://docs.substrate.io/reference/frame-macros/).
//!
//! ### Pallet Sections
//!
//! The pallet sections in this template are:
//!
//! - A **configuration trait** that defines the types and parameters which the pallet depends on
//!   (denoted by the `#[pallet::config]` attribute). See: [`Config`].
//! - A **means to store pallet-specific data** (denoted by the `#[pallet::storage]` attribute).
//!   See: [`storage_types`].
//! - A **declaration of the events** this pallet emits (denoted by the `#[pallet::event]`
//!   attribute). See: [`Event`].
//! - A **declaration of the errors** that this pallet can throw (denoted by the `#[pallet::error]`
//!   attribute). See: [`Error`].
//! - A **set of dispatchable functions** that define the pallet's functionality (denoted by the
//!   `#[pallet::call]` attribute). See: [`dispatchables`].
//!
//! Run `cargo doc --package pallet-template --open` to view this pallet's documentation.

// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// FRAME pallets require their own "mock runtimes" to be able to run unit tests. This module
// contains a mock runtime specific for testing this pallet's functionality.
#[cfg(test)]
mod mock;

// This module contains the unit tests for this pallet.
// Learn about pallet unit testing here: https://docs.substrate.io/test/unit-testing/
#[cfg(test)]
mod tests;

// Every callable function or "dispatchable" a pallet exposes must have weight values that correctly
// estimate a dispatchable's execution time. The benchmarking module is used to calculate weights
// for each dispatchable and generates this pallet's weight.rs file. Learn more about benchmarking here: https://docs.substrate.io/test/benchmark/
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;

use frame_support::traits::Time;
pub use weights::*;

// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet]
pub mod pallet {
    // Import various useful types required by all FRAME pallets.
    use super::*;
    use frame_support::{pallet_prelude::*, traits::Time, BoundedVec};
    use frame_system::pallet_prelude::*;

    // The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
    // (`Call`s) in this pallet.
    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// The pallet's configuration trait.
    ///
    /// All our types and constants a pallet depends on must be declared here.
    /// These types are defined generically and made concrete when the pallet is declared in the
    /// `runtime/src/lib.rs` file of your chain.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching runtime event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// A type representing the weights required by the dispatchables of this pallet.
        type WeightInfo: WeightInfo;

        type TimeProvider: Time;

        /// Maximum number of history versions per farm
        #[pallet::constant]
        type MaxHistoryPerFarm: Get<u32>;
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo, RuntimeDebug, MaxEncodedLen)]
    pub struct FarmAuditRecord<AccountId, Moment> {
        /// Raw farm data as compressed bytes (JSON, CBOR, or any format from backend)
        pub data: BoundedVec<u8, ConstU32<8192>>,
        /// Who made this update
        pub updated_by: AccountId,
        /// When this update was made
        pub updated_at: Moment,
        /// Optional: Data format identifier (0=JSON, 1=CBOR, etc.)
        pub data_format: u8,
    }

    /// A storage item for this pallet.
    ///
    /// In this template, we are declaring a storage item called `Something` that stores a single
    /// `u32` value. Learn more about runtime storage here: <https://docs.substrate.io/build/runtime-storage/>
    #[pallet::storage]
    pub type Something<T> = StorageValue<_, u32>;

    /// PRIMARY STORAGE: Main audit history
    /// Key: (farm_id, version_number) -> Value: FarmAuditRecord
    /// Supports Query 1: Get whole history by farm_id
    /// Supports Query 3: Get latest version by farm_id  
    #[pallet::storage]
    pub type FarmAuditHistory<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        u64, // farm_id (first key)
        Blake2_128Concat,
        u32, // version_number (second key)
        FarmAuditRecord<T::AccountId, <T::TimeProvider as Time>::Moment>,
    >;

    /// Track latest version number for each farm (Query 3 optimization)
    #[pallet::storage]
    pub type FarmVersionCounter<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u64,        // farm_id
        u32,        // latest_version
        ValueQuery, // Default to 0
    >;

    /// Events that functions in this pallet can emit.
    ///
    /// Events are a simple means of indicating to the outside world (such as dApps, chain explorers
    /// or other users) that some notable update in the runtime has occurred. In a FRAME pallet, the
    /// documentation for each event field and its parameters is added to a node's metadata so it
    /// can be used by external interfaces or tools.
    ///
    ///	The `generate_deposit` macro generates a function on `Pallet` called `deposit_event` which
    /// will convert the event type of your pallet into `RuntimeEvent` (declared in the pallet's
    /// [`Config`] trait) and deposit it using [`frame_system::Pallet::deposit_event`].
    #[pallet::event]
    //     you don’t see a manual function definition for deposit_event.
    // That’s because it’s generated by this macro:
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A user has successfully set a new value.
        SomethingStored {
            /// The new value set.
            something: u32,
            /// The account who set the new value.
            who: T::AccountId,
        },
        /// Farm data has been stored successfully
        FarmDataStored {
            farm_id: u64,
            version: u32,
            who: T::AccountId,
            timestamp: <T::TimeProvider as Time>::Moment,
        },
        /// Farm history queried
        FarmHistoryQueried {
            farm_id: u64,
            version: u32,
            who: T::AccountId,
        },
    }

    /// Errors that can be returned by this pallet.
    ///
    /// Errors tell users that something went wrong so it's important that their naming is
    /// informative. Similar to events, error documentation is added to a node's metadata so it's
    /// equally important that they have helpful documentation associated with them.
    ///
    /// This type of runtime error can be up to 4 bytes in size should you want to return additional
    /// information.
    #[pallet::error]
    pub enum Error<T> {
        /// The value retrieved was `None` as no value was previously set.
        NoneValue,
        /// There was an attempt to increment the value in storage over `u32::MAX`.
        StorageOverflow,
        /// Data size exceeds maximum allowed
        DataTooLarge,
        /// Maximum history limit reached for this farm
        MaxHistoryReached,
        /// Version not found
        VersionNotFound,
        /// Invalid data format
        InvalidDataFormat,
    }

    /// The pallet's dispatchable functions ([`Call`]s).
    ///
    /// Dispatchable functions allows users to interact with the pallet and invoke state changes.
    /// These functions materialize as "extrinsics", which are often compared to transactions.
    /// They must always return a `DispatchResult` and be annotated with a weight and call index.
    ///
    /// The [`call_index`] macro is used to explicitly
    /// define an index for calls in the [`Call`] enum. This is useful for pallets that may
    /// introduce new dispatchables over time. If the order of a dispatchable changes, its index
    /// will also change which will break backwards compatibility.
    ///
    /// The [`weight`] macro is used to assign a weight to each call.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// An example dispatchable that takes a single u32 value as a parameter, writes the value
        /// to storage and emits an event.
        ///
        /// It checks that the _origin_ for this call is _Signed_ and returns a dispatch
        /// error if it isn't. Learn more about origins here: <https://docs.substrate.io/build/origins/>
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::store_farm_update())]
        pub fn store_farm_update(
            origin: OriginFor<T>,
            farm_id: u64,
            data: BoundedVec<u8, ConstU32<8192>>,
            data_format: u8,
        ) -> DispatchResult {
            // Verify the transaction is signed
            let who = ensure_signed(origin)?;

            // Get current timestamp
            let current_time = T::TimeProvider::now();

            // Get current version and increment
            let current_version = FarmVersionCounter::<T>::get(farm_id);
            let new_version = current_version.saturating_add(1);

            // Check if we've reached the maximum history limit
            ensure!(
                new_version <= T::MaxHistoryPerFarm::get(),
                Error::<T>::MaxHistoryReached
            );

            // Create the audit record
            let audit_record = FarmAuditRecord {
                data,
                updated_by: who.clone(),
                updated_at: current_time,
                data_format,
            };

            // Store the audit record
            FarmAuditHistory::<T>::insert(farm_id, new_version, audit_record);

            // Update the version counter
            FarmVersionCounter::<T>::insert(farm_id, new_version);

            // Emit event
            Self::deposit_event(Event::FarmDataStored {
                farm_id,
                version: new_version,
                who,
                timestamp: current_time,
            });

            Ok(())
        }

        /// Get specific version of farm data
        ///
        /// This is a query function that doesn't modify state.
        /// In production, you'd typically use RPC calls for queries,
        /// but this demonstrates the pattern.
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::query_farm_version())]
        pub fn query_farm_version(
            origin: OriginFor<T>,
            farm_id: u64,
            version: u32,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Check if the version exists
            ensure!(
                FarmAuditHistory::<T>::contains_key(farm_id, version),
                Error::<T>::VersionNotFound
            );

            // Emit event to indicate query was performed
            Self::deposit_event(Event::FarmHistoryQueried {
                farm_id,
                version,
                who,
            });

            Ok(())
        }

        /// An example dispatchable that may throw a custom error.
        ///
        /// It checks that the caller is a signed origin and reads the current value from the
        /// `Something` storage item. If a current value exists, it is incremented by 1 and then
        /// written back to storage.
        ///
        /// ## Errors
        ///
        /// The function will return an error under the following conditions:
        ///
        /// - If no value has been set ([`Error::NoneValue`])
        /// - If incrementing the value in storage causes an arithmetic overflow
        ///   ([`Error::StorageOverflow`])
        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::cause_error())]
        pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            // Read a value from storage.
            match Something::<T>::get() {
                // Return an error if the value has not been set.
                None => Err(Error::<T>::NoneValue.into()),
                Some(old) => {
                    // Increment the value read from storage. This will cause an error in the event
                    // of overflow.
                    let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
                    // Update the value in storage with the incremented result.
                    Something::<T>::put(new);
                    Ok(())
                }
            }
        }
    }
}

// Helper functions (not dispatchables, for internal/RPC use)
impl<T: Config> Pallet<T> {
    /// Get the latest version number for a farm
    pub fn get_latest_version(farm_id: u64) -> u32 {
        FarmVersionCounter::<T>::get(farm_id)
    }

    /// Get specific farm version data
    pub fn get_farm_version(
        farm_id: u64,
        version: u32,
    ) -> Option<FarmAuditRecord<T::AccountId, <T::TimeProvider as Time>::Moment>> {
        FarmAuditHistory::<T>::get(farm_id, version)
    }

    /// Get complete farm history (all versions)
    pub fn get_farm_history(
        farm_id: u64,
    ) -> Vec<(
        u32,
        FarmAuditRecord<T::AccountId, <T::TimeProvider as Time>::Moment>,
    )> {
        let latest_version = Self::get_latest_version(farm_id);
        let mut history = Vec::new();

        for version in 1..=latest_version {
            if let Some(record) = FarmAuditHistory::<T>::get(farm_id, version) {
                history.push((version, record));
            }
        }

        history
    }

    /// Get farm history in a specific version range
    pub fn get_farm_history_range(
        farm_id: u64,
        start_version: u32,
        end_version: u32,
    ) -> Vec<(
        u32,
        FarmAuditRecord<T::AccountId, <T::TimeProvider as Time>::Moment>,
    )> {
        let mut history = Vec::new();

        for version in start_version..=end_version {
            if let Some(record) = FarmAuditHistory::<T>::get(farm_id, version) {
                history.push((version, record));
            }
        }

        history
    }

    /// Check if a farm has any history
    pub fn farm_exists(farm_id: u64) -> bool {
        FarmVersionCounter::<T>::get(farm_id) > 0
    }

    /// Get the latest farm data
    pub fn get_latest_farm_data(
        farm_id: u64,
    ) -> Option<FarmAuditRecord<T::AccountId, <T::TimeProvider as Time>::Moment>> {
        let latest_version = Self::get_latest_version(farm_id);
        if latest_version > 0 {
            FarmAuditHistory::<T>::get(farm_id, latest_version)
        } else {
            None
        }
    }

    /// Count total versions across all farms (expensive operation)
    pub fn count_total_versions() -> u32 {
        let mut count = 0;
        for (_, version_count) in FarmVersionCounter::<T>::iter() {
            count += version_count;
        }
        count
    }

    /// Get all farms that have history (returns farm_ids)
    pub fn get_all_farms() -> Vec<u64> {
        FarmVersionCounter::<T>::iter()
            .map(|(farm_id, _)| farm_id)
            .collect()
    }
}

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;
