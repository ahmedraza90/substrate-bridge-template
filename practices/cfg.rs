
#![cfg_attr(not(feature = "std"), no_std)]
// filepath: /home/devflare/bridge-temp/chain-b/pallets/template/src/lib.rs

## **What does it mean?**

- `#![...]` is a **crate-level attribute** (applies to the whole file/crate).
What does this include?
Everything in the file where the attribute is written.
All modules (mod mock;, mod tests;, etc.) declared in that file.
All child files that are part of the crate (via mod statements).

- `cfg_attr` means: "Apply an attribute only if a certain condition is true."
- `not(feature = "std")` means: "If the `std` feature is **not** enabled..."
- `no_std` means: "Do not use Rust's standard library."

## **Why is this used in Substrate pallets?**

- Substrate pallets are compiled to **WebAssembly (Wasm)** for blockchain runtimes.
- Wasm does **not** support Rust's standard library (`std`), only `core` and `alloc`.
- This line ensures your pallet can be compiled for Wasm (no standard library) **and** for normal Rust (with `std`) when testing or benchmarking.

cfg(feature = "runtime-benchmarks") means:
"Only include the following code if the Cargo feature runtime-benchmarks is enabled."

#[cfg(test)]
onlyrun this command on "cargo tes"

[cfg(feature = "runtime-benchmarks")]
Only include the following code if the Cargo feature runtime-benchmarks is enabled

✅ What is #[allow(...)] in Rust?
#[allow(...)] is an attribute that tells the Rust compiler:
"Please don't warn me about this thing, even if it normally would."
🔔 Examples of Compiler Warnings You Can Allow
Warning Name	What It Means
unused_variables/unused_imports	You declared a variable/imports but never used it
dead_code	You wrote code that is never used or called
non_snake_case	Your function/variable name doesn't follow snake_case style
missing_docs	Public items don’t have documentation comments

✅ Understanding #[derive_impl()] with a Simple Example
The #[derive_impl()] attribute is a Substrate-specific feature that makes implementing 
traits easier. 
Let me break it down with a simple example:
The main benefit of #[derive_impl()] is that you only need to define an implementation 
once, and then you can reuse it across multiple types.

// OR logic
#[cfg(any(unix, windows))] 
fn os_specific() {}

// Single condition
#[cfg(debug_assertions)] 
fn debug_only() {}