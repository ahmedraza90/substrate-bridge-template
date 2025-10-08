Steps for Validators Selecting and Voting on a Chain
Step 1: Observe Available Chains (Local View)

What happens: Each validator receives blocks produced by BABE (Polkadot’s block production mechanism) via network gossip. These blocks form one or more chains (due to potential forks).
Why different views?

Network delays: Blocks propagate at different speeds. Validator V1 might see Chain A (Block #99 → #100 → #101), while V3 only sees Chain B (Block #99 → #100’) due to slower network updates.
Forks: Multiple validators might produce blocks at the same height (e.g., Block #100 and #100’ after #99), creating temporary forks.
Temporary divergence: Validators temporarily see different chains because of forks or delays, leading to different local views.


Outcome: Each validator has a local view of one or more chains (e.g., Chain A, Chain B).

Step 2: Pick the “Best” Chain (Fork-Choice Rule: GHOST)

What happens: Validators apply the GHOST (Greedy Heaviest Observed SubTree) rule to select the “best” chain from their local view.
Why the longest/heaviest chain?

Why the highest block? GRANDPA aims to finalize the longest valid chain to ensure the blockchain progresses and doesn’t get stuck on older blocks. Voting on the highest valid block helps validators converge on the most up-to-date chain.

The longest chain (with the most blocks, especially BABE’s primary blocks) represents the most validator work and network agreement, making it likely the canonical chain.
“Heaviest” refers to the chain with the most weight (based on BABE block production, prioritizing primary blocks over secondary ones).


Example:

Chain A: Block #99 → #100 → #101 (3 blocks, hash: H101).
Chain B: Block #99 → #100’ (2 blocks, hash: H100’).
V1 and V2 pick Chain A because it’s longer (more blocks). V3 picks Chain B because it hasn’t seen #101 yet (network delay).


Outcome: Each validator selects the highest block in their chosen chain (e.g., V1, V2 pick H101; V3 picks H100’).

Step 3: Check Chain Validity Before Voting

What happens: Before voting on the highest block’s hash, validators verify the chain is valid to ensure they’re not endorsing an invalid or malicious chain.
What they check:

Block Validity (for each block in the chain):

The block header is correct (parent_hash, state_root, extrinsics_root).
Transactions (extrinsics) produce the claimed state_root when executed (e.g., balance updates are correct).
The block follows consensus rules (e.g., no invalid transactions or state changes).


Chain Consistency:

Each block’s parent_hash links to a valid previous block, forming a consistent chain back to the last finalized block.
The chain doesn’t violate rules (e.g., no double-spending).




Why check? A longer chain could be invalid (e.g., fake transactions). Validators ensure the chain is trustworthy before voting.
Outcome: Validators confirm their chosen chain (e.g., Chain A for V1, V2; Chain B for V3) is valid.

Step 4: Vote on the Highest Block’s Hash

What happens: Each validator signs the hash of the highest block in their chosen chain (e.g., V1, V2 sign H101; V3 signs H100’) and broadcasts the vote.
Details:

The vote includes (ValidatorId, Signature) for the block hash.
Voting on a block hash implicitly endorses all ancestor blocks (e.g., voting for H101 endorses #101, #100, #99).


Outcome: Votes are collected in a GRANDPA voting round (e.g., Round 1: H101 gets 2 votes, H100’ gets 1 vote).

Step 5: Resolve Forks and Reach Consensus (Multiple Rounds)

What happens: GRANDPA runs voting rounds to achieve a 2/3+ supermajority.
Why multiple rounds?

Forks and temporary divergence: Validators may vote for different chains (e.g., Chain A vs. Chain B) due to network delays or forks.
Convergence: Validators see others’ votes and may switch to a longer valid chain in the next round (e.g., V3 sees H101 votes, verifies Chain A, and switches to vote for H101).


Example:

Round 1: V1, V2 vote for H101 (Chain A); V3 votes for H100’ (Chain B). No supermajority (2/3 of 3 = 2 votes isn’t enough for H101).
Round 2: V3 verifies Chain A (#101), switches to vote for H101. Now H101 gets 3 votes (supermajority).


Outcome: The highest common block (e.g., #101) is finalized, along with all its ancestors (#100, #99).

Step 6: Generate Finality Proof

What happens: Once a supermajority agrees on a block (e.g., #101), GRANDPA generates a FinalityProof:

block_hash: The finalized block’s hash (e.g., H101).
justification (GrandpaJustification): Includes round_number, validator signatures, and optionally maybe_votes or maybe_set_id (if validator set changed).


Purpose: The FinalityProof proves the block is finalized (permanent) to other nodes or chains (e.g., Rialto in a Millau-Rialto setup).
Outcome: The block is irreversible, its state (e.g., balances) is locked in, and it’s trusted across the network.

Why These Steps Handle Forks, Delays, and Divergence?

Forks: Occur when multiple validators produce blocks at the same height (e.g., #100 and #100’). GRANDPA’s voting resolves this by picking the chain with the most validator support.
Network Delays: Cause validators to see different chains (e.g., V3 misses #101). Multiple rounds allow them to catch up and align.
Temporary Divergence: Validators start with different views but converge through voting rounds by verifying and switching to the longest valid chain.

Simple Example

Chain A: Block #99 → Block #100 → Block #101 (hash: H101)
Chain B: Block #99 → Block #100’ (hash: H100’)


Validator V1 sees Chain A and votes for H101 (highest block).
Validator V2 sees Chain A and also votes for H101.
Validator V3 sees Chain B and votes for H100’ (highest block in Chain B).
Round 1 Outcome:

Votes: H101 (2 votes), H100’ (1 vote).
No 2/3+ supermajority yet (2/3 of 3 validators = 2 votes isn’t enough for H101).


Round 2:

V3 sees votes for H101 and checks Chain A. If it’s valid, V3 switches to vote for H101.
Votes: H101 (3 votes).
Supermajority achieved! GRANDPA finalizes Block #101 (and all ancestors, e.g., #100, #99).


FinalityProof: Generated with block_hash: H101 and signatures from V1, V2, V3. This proves Block #101 is final.