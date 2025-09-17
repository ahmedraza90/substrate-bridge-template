// 🔹 Step 1: Transaction validation (mempool)

// Who does it? Any full node (not just validators).

// What is checked?

// Signature is correct.

// Nonce is correct (tx is not a replay).

// Sender has enough balance for fees.

// The tx looks valid but is not actually applied to state yet.

// 👉 At this point, the node just says: “This transaction is syntactically and logically valid, so I’ll store it in my mempool and gossip it.”
// It does not deduct balances or change state yet. It’s only a pre-check.

// 🔹 Step 2: Block production (validator executes)

// Who does it? The validator chosen by BABE for that slot.

// What happens?

// The validator picks transactions from its mempool.

// For each tx, it actually runs the code and applies it to state:

// Deducts Alice’s balance.

// Adds Bob’s balance.

// Updates storage root.

// While doing this, it tracks the computation cost (ref_time).

// Stops once block weight limit is reached.

// 👉 Here, the transactions are really executed on-chain.
// The block now contains {extrinsics, new state root, …} and is broadcast.
