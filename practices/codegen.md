# Start Chain A node with custom ports
./target/release/solochain-template-node --dev --rpc-port 9944 --rpc-external --port 30334

# Start Chain B node with different ports (avoid conflicts)

./target/release/solochain-template-node --dev --rpc-port 9945 --rpc-external --port 30333

*** What each flag does ***:

--dev: Starts the node with a development chain specification: a single validator node, no network connections, instant block production (no consensus delays), and fast finality. Great for local testing.
--rpc-port: WebSocket RPC port for API connections
--rpc-external: Allow external connections (not just localhost)
--port: P2P networking port for blockchain communication
--tmp	Uses a temporary database directory, which is automatically deleted when the node shuts down. This ensures no blockchain state or storage is persisted across runs.

# Generate Rust types for Chain A runtime (from running node)
subxt codegen --url ws://localhost:9944 > chain_a_codegen_runtime.rs

# Generate Rust types for Chain B runtime - NOTE: filename should be different!
subxt codegen --url ws://localhost:9945 > chain_b_codegen_runtime.rs


*** What this generates ***:

Type-safe Rust structs for all pallets
Function calls for all extrinsics
Storage queries for all storage items
Event types for all events

# Extract raw metadata from Chain A (binary format for analysis)
subxt metadata --url ws://localhost:9944 --format bytes > metadata.scale

*** Use cases ***:

Debugging runtime issues
Comparing metadata between chains
Manual metadata analysis


# 1️⃣ What is metadata.scale?
It’s the raw runtime metadata of a Substrate chain, dumped in SCALE-encoded bytes.

Think of it as a blueprint of everything your blockchain knows about itself:

All pallets (balances, timestamp, staking, your custom ones…)
All extrinsics (callable functions)
All storage items (key-value maps in pallets)
All events & constants

# When you run:
subxt metadata --url ws://localhost:9944 --format bytes > metadata.scale


You’re asking Subxt:
👉 “Give me the low-level runtime metadata of this chain, exactly as the node provides it, without converting it into Rust yet.”

# 📌 Use cases:

Debugging differences between two chains’ runtimes
Saving a “snapshot” of metadata for later analysis
Feeding into tools that can decode SCALE format