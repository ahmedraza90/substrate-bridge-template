// 1️⃣ Does each pallet have its own RocksDB storage?

// No — all pallets share the same RocksDB database.
// What’s different is how keys are organized so that pallets don’t overwrite each other’s data.

// Think of RocksDB as one big key-value store:

// The key is a Vec<u8> (bytes) that contains:

// The pallet’s unique storage prefix (e.g., "Balances").

// The storage item name (e.g., "FreeBalance").

// Any additional key data (like account IDs).

// The value is the SCALE-encoded value for that storage item.

// Example:

// Key:   "Balances" + "FreeBalance" + <AccountId>
// Value: SCALE(1000)

// So while every pallet feels like it has “its own” storage, technically they’re all writing into the same RocksDB instance, but in different namespaces.

// ow Substrate state is “versioned” per block

// Unlike a normal database where there’s only one current state,
// Substrate keeps a versioned state — one full snapshot for each block.

// It’s not a literal “copy” of the entire DB for each block (that would be huge).
// Instead, it uses a Merkle–Patricia Trie (MPT), where unchanged data is reused and only modified parts are stored again.

// 🖼 Visualization
// Block #1000
//  └── MPT Root Hash: 0xaaa
//       ├── pallet_balances:
//       │      Alice: 10
//       │      Bob: 50
//       └── pallet_staking:
//              ValidatorCount: 4

// Block #1001
//  └── MPT Root Hash: 0xbbbb
//       ├── pallet_balances:
//       │      Alice: 25   <-- changed (new node in trie)
//       │      Bob: 50     <-- reused from block #1000
//       └── pallet_staking:
//              ValidatorCount: 4  <-- reused

// Block #1002
//  └── MPT Root Hash: 0xcccc
//       ├── pallet_balances:
//       │      Alice: 42   <-- changed again
//       │      Bob: 50     <-- still same as before
//       └── pallet_staking:
//              ValidatorCount: 5  <-- changed

// 3️⃣ What this means

// Each block’s state root (hash at the top of the trie) is stored in the block header.

// If any piece of state changes, that hash changes.

// You can go back in history and query state as it was at any block number.

// Old unchanged data is shared between states, so it’s storage-efficient.

// step-by-step through how the Merkle–Patricia Trie (MPT) changes when only one value changes, so you can see the “versioned state” magic.

// Step 1 — Block #1000 initial state

// We have two pallets:

// pallet_balances → Alice: 10, Bob: 50

// pallet_staking → ValidatorCount: 4

// MPT Root (hash: 0xAAA)
// ├── balances (hash: 0x111)
// │     ├── Alice: 10
// │     └── Bob: 50
// └── staking (hash: 0x222)
//       └── ValidatorCount: 4

// Step 2 — Block #1001 changes

// Only Alice’s balance changes from 10 → 25.

// MPT does not rebuild the whole tree.
// Instead:

// Alice’s leaf node changes (value 25 now)

// That changes the hash of the balances branch (0x111 → 0x333)

// That changes the root hash (0xAAA → 0xBBB)

// MPT Root (hash: 0xBBB)
// ├── balances (hash: 0x333)   <-- new
// │     ├── Alice: 25          <-- changed
// │     └── Bob: 50            <-- reused
// └── staking (hash: 0x222)    <-- reused
//       └── ValidatorCount: 4

// Step 3 — Block #1002 changes

// ValidatorCount changes from 4 → 5.

// Now:

// That leaf node changes

// The staking branch hash changes

// The root hash changes again

// MPT Root (hash: 0xCCC)
// ├── balances (hash: 0x333)   <-- reused from block #1001
// │     ├── Alice: 25
// │     └── Bob: 50
// └── staking (hash: 0x444)    <-- new
//       └── ValidatorCount: 5  <-- changed

// Key insight

// Substrate reuses unchanged branches between block states.

// This is why it can keep full history of state for every block without duplicating the entire DB.

// The RocksDB stores all these trie nodes, each keyed by their hash.
// If a node doesn’t change, the same hash is used in multiple blocks.
