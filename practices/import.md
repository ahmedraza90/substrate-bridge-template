**Binary Crate vs Library Crate**

Binary Crate (executable program):

Has a main.rs file with a main() function
Can be run directly: cargo run
Produces an executable file you can run
Example: A web server, CLI tool, game
Cannot be imported by other projects

Library Crate (reusable code):

**Has a lib.rs file (no main() function)
Cannot be run directly
Provides code that other projects can use
Example: serde, tokio, axum - these are all libraries
Can be imported by other projects with use axum::...

**Import Behavior Difference**
In a binary crate, main.rs is the crate root. 

Any imprt by "use" statements in main.rs becomes available at for the whole project
//EXAMPLE:
// src/main.rs (this is your crate root)

mod state; // declaring the module it means This binary needs the state module
use crate::state::AppState;  // Re-export at root // it will be available to whole project 

// src/handlers/task.rs
use crate::AppState;  // Works! Because main.rs re-exported it // No need to write full path again and again

in library lib.rs do the same thing:
re-exports inside lib.rs becomes available for the whole project:

**FOR FOLDER OUTSIDE OF PROJECT LIKE TEST FOLDER:**
Simple Rule:
Binary crate (main.rs only): Code is private, only for that program
Library crate (lib.rs): Code is public, can be shared with tests and other projects
Both: Library has the reusable code, binary is a thin wrapper that runs it

**SINGLE PROJECT CAN BE BOTH:**
we can have BOTH a binary crate AND a library crate at the same time!

Binary crate (src/main.rs): Still exists, still runnable with cargo run
Library crate (src/lib.rs): Newly added, used by tests and the binary itself

**[lib]**
🧠 In Rust, a single Cargo project can build multiple “targets”:

🧩Binary target → comes from src/main.rs
It produces an executable (cargo run runs this).
🧩Library target → comes from src/lib.rs (when declared with [lib])
It produces a library crate that other code can import (like your integration tests).

That library target becomes available as a crate named after your project:
use axum_prac::config::Config;

💡 Without the [lib] section, your code only compiles into an executable — not something testable/importable by other crates.

**The Key Point:**
When you have both src/main.rs AND src/lib.rs:

For the library part (everything in src except main.rs): src/lib.rs is the crate root
For the binary part (src/main.rs only): src/main.rs is still the crate root of the binary

📁 axum-prac
 ├── 📁 src
 │    ├── lib.rs
 │    ├── main.rs
 │    ├── 📁 config
 │    ├── 📁 errors
 │    ├── 📁 handlers
 │    ├── 📁 models
 │    ├── 📁 routes
 │    └── 📁 state
 ├── 📁 tests
 │    └── api_test.rs
 ├── Cargo.toml

**⚙️ [dev-dependencies] — what it means**

This section is about build scope, not file location.
Rust has two kinds of dependencies in Cargo.toml:

Section	            Purpose
[dependencies]	    For normal build (main app, library, etc.)
[dev-dependencies]	Only compiled when you run cargo test, cargo bench, or build examples






This is a great example of how Rust's **module system** and **import resolution** works. Let me explain why the imports aren't needed in `configs/mod.rs`:

## **The Magic: How Rust Resolves Types Without Explicit Imports**0

### **1. Parent Module Imports**

In your `lib.rs`, you already have:

```rust
// lib.rs
pub use frame_system::Call as SystemCall;
pub use pallet_balances::Call as BalancesCall;
pub use pallet_timestamp::Call as TimestampCall;

// Bridge imports
use bridge_config::{
    BridgeGrandpaInstance, 
    BridgeMessagesInstance, 
    BridgeRelayersInstance,
    CrossChainMessageDispatcher,
    UniversalChain,
};
```

### **2. Cargo.toml Dependencies**

In your `runtime/Cargo.toml`, you have:

```toml
[dependencies]
pallet-bridge-grandpa.workspace = true
pallet-bridge-messages.workspace = true
pallet-bridge-relayers.workspace = true
# ... etc
```

### **3. Module Hierarchy**

```
lib.rs (parent)
└── configs/mod.rs (child)
```

When `configs/mod.rs` is a **child module** of `lib.rs`, it **inherits the parent's scope**.

---

## **How Rust Finds Types in `configs/mod.rs`**

### **Example 1: Basic Pallet Types**

```rust
// In configs/mod.rs
impl pallet_bridge_grandpa::Config<BridgeGrandpaInstance> for Runtime {
    //   ^^^^^^^^^^^^^^^^^^^^^ How does Rust find this?
```

**Rust's Search Process:**
1. **Current module scope** → Not found in `configs/mod.rs`
2. **Parent module scope** → Check `lib.rs` 
3. **Crate dependencies** → Found in `Cargo.toml` dependencies!
4. **Rust standard library** → (if not found above)

### **Example 2: Bridge Instance Types**

```rust
// In configs/mod.rs  
impl pallet_bridge_grandpa::Config<BridgeGrandpaInstance> for Runtime {
    //                              ^^^^^^^^^^^^^^^^^^^ How does Rust find this?
```

**Rust's Search Process:**
1. **Current module** → Not in `configs/mod.rs`
2. **Parent module** → Found! `lib.rs` imports it from `bridge_config`
3. ✅ **Success!**

---

## **Real-World Analogy**

Think of it like a **family house**:

### **🏠 House Structure:**
```
🏠 lib.rs (Parent's House)
├── 📚 All the books (dependencies)
├── 🔑 Keys to other rooms (imports)
└── 🚪 configs/mod.rs (Child's Room)
```

**Child's Room (configs/mod.rs):**
- Child wants to use a book 📖
- Child doesn't have the book in their room
- Child can access **parent's bookshelf** (lib.rs imports)
- Child can also access **house library** (Cargo.toml dependencies)

---

## **Demonstration: What's Actually Happening**

Let me show you the **implicit resolution**:

### **What You Write:**
```rust
// configs/mod.rs
impl pallet_bridge_grandpa::Config<BridgeGrandpaInstance> for Runtime {
    type RuntimeEvent = RuntimeEvent;
}
```

### **What Rust Actually Resolves:**
```rust
// What Rust "sees" after resolution
impl crate::pallet_bridge_grandpa::Config<crate::bridge_config::BridgeGrandpaInstance> for crate::Runtime {
    type RuntimeEvent = crate::RuntimeEvent;
}
```

---

## **Experiment: Proof This Works**

Try this experiment to see the difference:

### **Test 1: Remove Parent Import**

**Step 1:** Comment out this line in `lib.rs`:
```rust
// lib.rs
// use bridge_config::BridgeGrandpaInstance;  // ❌ Comment this out
```

**Step 2:** Try to compile:
```bash
cargo check
```

**Result:** ❌ **Error!** 
```
error[E0433]: failed to resolve: use of undeclared type `BridgeGrandpaInstance`
```

### **Test 2: Add Direct Import**

**Step 1:** Add import to `configs/mod.rs`:
```rust
// configs/mod.rs
use crate::bridge_config::BridgeGrandpaInstance;  // ✅ Add this

impl pallet_bridge_grandpa::Config<BridgeGrandpaInstance> for Runtime {
    // ...
}
```

**Step 2:** Compile:
```bash
cargo check
```

**Result:** ✅ **Success!** Now it works even without parent import.

---

## **Why Your Current Setup Works**

### **✅ Working Resolution Chain:**

1. **Cargo Dependencies** → `pallet_bridge_grandpa` is in `Cargo.toml`
2. **Parent Imports** → `BridgeGrandpaInstance` imported in `lib.rs`  
3. **Module Hierarchy** → `configs/mod.rs` inherits from `lib.rs`
4. **Type Resolution** → Rust finds everything automatically!

### **📋 Summary Table:**

| Type | Found Where | How |
|------|-------------|-----|
| `pallet_bridge_grandpa` | Cargo.toml dependencies | Auto-imported |
| `BridgeGrandpaInstance` | lib.rs imports | Inherited by child module |
| `Runtime` | lib.rs definition | Inherited by child module |
| `RuntimeEvent` | Generated by runtime macro | Inherited by child module |

---

## **Key Rust Concepts**

### **1. Module Privacy Rules:**
- Child modules can access parent's **public** items
- Child modules automatically inherit parent's **use** statements

### **2. Dependency Resolution:**
- Cargo dependencies are **globally accessible** within the crate
- No need to import crate dependencies explicitly (like `pallet_bridge_grandpa`)

### **3. Scope Inheritance:**
- `configs/mod.rs` inherits everything from `lib.rs`
- This is why you don't see imports in `configs/mod.rs`

---

This is a **beautiful example** of Rust's intelligent module system working behind the scenes! The language is designed to minimize boilerplate while maintaining type safety.

Your setup works because Rust **automatically resolves** types through the module hierarchy and dependency system. Pretty elegant, right? 🎯


4. Re-exports (pub use)

A shortcut to make items from a module available at the crate root.

Example:

pub mod animals {
    pub struct Dog;
}

// Re-export
pub use animals::*;


Now you can do:

use crate::Dog;              // ✅ shortcut
use crate::animals::Dog;     // ✅ still works
If you don't re-export, you can still use those items, but you must use the full path, like crate::mod::Item.

crate refers to the current crate (the whole Rust package, which is your pallet).