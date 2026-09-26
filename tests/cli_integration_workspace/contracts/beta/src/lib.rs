#![no_std]

//! Beta Contract
//!
//! This module contains a simple demonstration contract for the Soroban network.
//! It is primarily used within the integration test workspace to verify that
//! the linter correctly analyzes contracts without loops and standard storage operations.

use soroban_sdk::{symbol_short, Env, Symbol};

/// The symbol key used to store our safe value in the contract's instance storage.
const SAFE: Symbol = symbol_short!("safe");

/// A simple function that sets a value in instance storage without any loops.
///
/// This function is intended to demonstrate a "safe" operation that should not
/// trigger any excessive cost warnings from the linter. It simply assigns the
/// integer `42` to the `SAFE` symbol key.
///
/// # Arguments
///
/// * `env` - The Soroban environment, providing access to storage and other host functions.
pub fn good_no_loop(env: Env) {
    env.storage().instance().set(&SAFE, &42i32);
}
