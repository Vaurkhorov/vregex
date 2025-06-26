use crate::types::re;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// WASM wrapper for the `RegEx` struct
pub struct RegEx {
    engine: re::RegEx,
}

#[wasm_bindgen]
impl RegEx {
    #[wasm_bindgen(constructor)]
    pub fn new(pattern: &str) -> Result<Self, String> {
        Ok(Self {
            engine: re::RegEx::from_pattern(pattern).map_err(|e| format!("{:#?}", e))?,
        })
    }

    #[wasm_bindgen]
    pub fn search(&self, string: &str) -> Option<u32> {
        self.engine.search(string).map(|v| v as u32)
    }
}
