#[cfg(target_arch = "wasm32")]
use wasm_minimal_protocol::wasm_func;

pub mod ast;
pub mod parser;

#[cfg(target_arch = "wasm32")]
wasm_minimal_protocol::initiate_protocol!();

#[cfg_attr(target_arch = "wasm32", wasm_func)]
pub fn concatenate(arg1: &[u8], arg2: &[u8]) -> Vec<u8> {
    [arg1, arg2].concat()
}
