#![no_std]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// these two are conditionally compiled, only for wasm32
pub mod exports;
pub mod imports;

pub mod errors;
pub mod memory;
pub mod mock;
pub mod serde;
pub mod storage;
pub mod types;
