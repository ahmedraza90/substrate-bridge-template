Based on the **`traits.rs`** file, here's a detailed table of the `Client<C: Chain>` interface:

## Client Trait Interface Methods

| **Category** | **Method** | **Description** | **Purpose** |
|-------------|------------|-----------------|-------------|
| **Connection Management** | `ensure_synced()` | Checks if client is connected and synced with the chain | Ensures relayer has up-to-date chain data |
| | `reconnect()` | Reconnects to the chain node | Handles connection failures |
| **Basic Chain Info** | `genesis_hash()` | Returns the hash of the first block | Chain identification |
| | `header_hash_by_number()` | Gets block hash by block number | Find specific blocks |
| | `header_by_hash()` | Gets block header by its hash | Retrieve block metadata |
| | `header_by_number()` | Gets block header by block number | Retrieve block metadata by number |
| | `block_by_hash()` | Gets complete block data by hash | Access full block content |
| **Finalized Chain State** | `best_finalized_header_hash()` | Hash of latest finalized block | Track confirmed blocks |
| | `best_finalized_header_number()` | Number of latest finalized block | Track chain progress |
| | `best_finalized_header()` | Full header of latest finalized block | Get finalized block details |
| **Latest Chain State** | `best_header()` | Latest block header (may not be finalized) | Track chain tip |
| | `best_header_hash()` | Hash of latest block | Quick chain tip reference |
| **Subscriptions** | `subscribe_best_headers()` | Live updates of new block headers | Real-time chain monitoring |
| | `subscribe_finalized_headers()` | Live updates of finalized headers | Track finality progress |
| | `subscribe_grandpa_finality_justifications()` | GRANDPA finality proofs | Monitor consensus finality |
| | `subscribe_beefy_finality_justifications()` | BEEFY finality proofs | Monitor BEEFY consensus |
| **GRANDPA Consensus** | `generate_grandpa_key_ownership_proof()` | Creates proof of validator authority | Verify validator permissions |
| **Chain Properties** | `token_decimals()` | Number of decimal places for native token | Token amount calculations |
| | `runtime_version()` | Full runtime version info | Compatibility checking |
| | `simple_runtime_version()` | Basic version for transaction signing | Transaction compatibility |
| | `can_start_version_guard()` | Check if version monitoring should run | Version management |
| **Storage Access** | `raw_storage_value()` | Read raw bytes from chain storage | Low-level storage access |
| | `storage_value()` | Read and decode storage value | Type-safe storage reading |
| | `storage_map_value()` | Read from storage map (key-value) | Access pallet storage maps |
| | `storage_double_map_value()` | Read from storage double map | Access complex storage structures |
| **Transaction Pool** | `pending_extrinsics()` | Get transactions waiting to be included | Monitor transaction queue |
| | `submit_unsigned_extrinsic()` | Submit transaction without signature | Submit system transactions |
| | `submit_signed_extrinsic()` | Submit transaction with signature | Submit user transactions |
| | `submit_and_watch_signed_extrinsic()` | Submit and track transaction status | Monitor transaction lifecycle |
| **Transaction Validation** | `validate_transaction()` | Check if transaction is valid | Pre-submission validation |
| | `estimate_extrinsic_weight()` | Calculate transaction execution cost | Fee estimation |
| **Runtime Calls** | `raw_state_call()` | Execute runtime function (raw) | Low-level runtime interaction |
| | `state_call()` | Execute runtime function (typed) | Type-safe runtime calls |
| **Storage Proofs** | `prove_storage()` | Generate cryptographic proof of storage | Cross-chain verification |

## Key Features for Bridge Relayers

1. **Real-time Monitoring**: Subscribe to new blocks and finality events
2. **Cross-chain Verification**: Generate storage proofs for other chains
3. **Transaction Management**: Submit and track bridge transactions
4. **Storage Access**: Read bridge pallet state from both chains
5. **Consensus Integration**: Handle GRANDPA and BEEFY finality proofs
6. **Type Safety**: Encode/decode data according to chain specifications

This interface provides everything needed for a bridge relayer to monitor two chains, read their state, verify finality, and submit cross-chain transactions.