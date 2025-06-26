mod types;

use types::ast::*;
use types::enfa::*;
use types::error::Error;

#[cfg(not(any(feature = "wasm", target_arch = "wasm32")))]
pub mod lib_native;

#[cfg(any(feature = "wasm", target_arch = "wasm32"))]
pub mod lib_wasm;
