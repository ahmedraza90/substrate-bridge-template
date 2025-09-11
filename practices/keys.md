# Substrate Keys and Addresses Explained

## Core Concepts

### Private Key vs Public Key vs Account Address and their derivation.

```
[Seed Phrase / Secret] → Private Key → Public Key → Account Address (AccountId32) → SS58 Address (human-readable format)
                              ↓            ↓                ↓
                           (signs)     (verifies)     (identifies)
```

- **Private Key**: Used to sign messages/transactions (kept secret)
- **Public Key**: Used to verify signatures (can be shared)  
- **Account Address**: Derived from public key, identifies accounts on-chain

- **The seed** (like "//Alice" or a mnemonic) is used to generate a private key.
- **The private key** is used to derive the public key (via elliptic curve cryptography)cryptographic randomness.
- **Then, the public key** is used to derive the AccountId32 (your wallet address on-chain).

---

## Key Types in Substrate

### 🧱 AccountId32
- A **32-byte unique identifier** for blockchain accounts
- Think of it as your full wallet address:
  ```
  0x1234abcd...7890ef (32 bytes total)
  ```
- **Always** represents the complete identity of a user on-chain
- **Immutable**: One public key = one AccountId32

### 🧮 MultiAddress  
- An **enum** that provides **flexible addressing formats**
- Allows different ways to reference the same account

```rust
enum MultiAddress<AccountId, Index> {
    Id(AccountId),        // Full address (AccountId32)
    Index(Index),         // Short numeric index (e.g., 0 = Alice)
    Raw([u8; 32]),        // Raw 32 bytes
    Address32([u8; 32]),  // Same as Raw but more explicit
    Address20([u8; 20]),  // Ethereum-style addresses (20 bytes)
}
```

**Why MultiAddress?**
- **Efficiency**: Use short indices instead of full 32-byte addresses
- **Compatibility**: Support different address formats (Ethereum, etc.)
- **Flexibility**: Choose the best format for your use case

---

## Creating Signers with Pair

### Basic Usage
```rust
let signer: Pair = Pair::from_string("//Alice", None)?;
```

### What `from_string()` Does:
1. **Parses the seed string** into cryptographic material
2. **Generates the private key** deterministically  
3. **Derives the public key** from the private key
4. **Returns a `Pair`** containing both keys

### Supported Seed Formats:

| Format                    | Example                 | Description                   |
|---------------------------|-------------------------|-------------                  |
| **Dev Seeds**             | `"//Alice"`, `"//Bob"`  | Built-in development accounts |
| **Hex Private Key**       | `"0x1234abcd..."`       | Direct private key            |
| **Mnemonic**              | `"word1 word2 ..."`     | BIP39 seed phrase             |
| **URI**                   | `"//Alice/stash"`       | Hierarchical derivation       |

### Development Seeds
```rust
"//Alice"  → Always generates Alice's keypair
"//Bob"    → Always generates Bob's keypair  
"//Charlie" → Always generates Charlie's keypair
```
- **Deterministic**: Same seed = same keys (every time)
- **For testing only**: Never use in production!

---

## Step-by-Step Key Flow

| Step | Code | What It Does |
|------|------|--------------|
| **1️⃣** | `Pair::from_string("//Alice", None)?` | Creates keypair from seed string |
| **2️⃣** | `signer` | Holds both private and public keys |
| **3️⃣** | `signer.public()` | Extracts the public key |
| **4️⃣** | `AccountId32::from(signer.public())` | Derives the blockchain address |

### Code Example:
```rust
use sp_core::{sr25519::Pair, Pair as PairTrait};
use sp_runtime::AccountId32;

// Create signer from seed
let signer = Pair::from_string("//Alice", None)?;

// Extract components
let private_key = &signer;           // Private key (for signing)
let public_key = signer.public();    // Public key (for verification)
let account_id = AccountId32::from(public_key); // Address (for identification)

println!("Account ID: {:?}", account_id);
```

---

## Key Relationships
```
    Seed String ("//Alice")
           ↓
    Pair::from_string()
           ↓
    ┌─────────────────┐
    │      Pair       │
    │  ┌─────────────┐│
    │  │ Private Key ││ → Signs transactions
    │  └─────────────┘│
    │  ┌─────────────┐│
    │  │ Public Key  ││ → Verifies signatures
    │  └─────────────┘│
    └─────────────────┘
           ↓
    AccountId32::from()
           ↓
    Account Address (32 bytes)
```

---

## Common Patterns

### Error Handling
```rust
let signer = match Pair::from_string(&signer_seed, None) {
    Ok(pair) => pair,
    Err(_) => return Err("Invalid signer seed"),
};
```

### Getting Account ID
```rust
// Method 1: Explicit conversion
let account_id = AccountId32::from(signer.public());

// Method 2: Using trait (if available)
let account_id = signer.to_account_id();
```

### Using MultiAddress
```rust
// Full address
let multi_addr = MultiAddress::Id(account_id);

// Short index (if supported)
let multi_addr = MultiAddress::Index(0); // Alice = 0
```

---

## Security Notes

⚠️ **Important**: 
- **Never use dev seeds** (`//Alice`, `//Bob`) in production
- **Keep private keys secure** - they can't be recovered if lost
- **Validate seed strings** before creating signers
- **Use proper error handling** for invalid seeds

✅ **Best Practices**:
- Generate random seeds for production accounts
- Use hardware wallets for high-value accounts  
- Implement proper key management systems
- Always validate signatures before processing transactions