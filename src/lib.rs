pub mod config;
pub mod parser;
pub mod util;
pub use htmlentity::entity;
#[cfg(target_arch = "wasm32")]
pub mod wasm;
#[cfg(target_arch = "wasm32")]
pub mod wasm_config;
