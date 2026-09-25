# `large_constant_array`

**Default Severity:** `warn`

**Target Resource:** [Memory — contract storage footprint](../cost_rationale.md#per-lint-resource-summary)

## What it does

Flags large constant array definitions, repeat expressions, and array literals
in contract code whose footprint exceeds a specific size threshold (currently **4 096 elements**).

## Why is this bad?

{% hint style="danger" %}
Embedding a large constant array directly in the contract source (e.g., `[0u8; 5000]`) 
increases the final WebAssembly (WASM) binary size linearly with the array length. 
Soroban heavily restricts and meters the size of deployed contracts. Large
binaries not only cost more to deploy but also cost more gas to load into 
memory upon every invocation. Data this large is often better managed 
dynamically using persistent storage or `Bytes`.
{% endhint %}

## Example

```rust
// ❌ Bad: the array is heavily bloated and inflates the binary size
const LARGE_DATA: [u8; 5000] = [0; 5000];

fn uses_data() {
    let buf = [0u8; 5000]; // Also bad
}
```

```rust
// ✅ Good: keeping the contract small and managing large data dynamically
fn uses_data(env: &Env) {
    let buf = env.storage().persistent().get(&symbol_short!("Data")).unwrap();
}
```

## Suggested Fix

{% hint style="success" }
Move large, static or semi-static datasets into Soroban persistent storage
or host-managed `Bytes`. Instead of bundling data in the Wasm, initialize 
it via an admin endpoint after deployment and read it dynamically, or accept 
the data as an invocation argument.
{% endhint %}
