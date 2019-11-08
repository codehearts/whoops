//! A TV series representation which is exported to wasm

#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(missing_docs)]

use wasm_bindgen::prelude::*;

/// Representation of a TV series
#[wasm_bindgen]
pub struct Series {
    /// The name of the series
    name: String,
    /// The total number of seasons for the series
    pub seasons: u8,
}

#[wasm_bindgen]
impl Series {
    #[wasm_bindgen(js_name=okKo)]
    /// Produces a `Series` for OK K.O.!
    pub fn ok_ko() -> Self {
        Self {
            name: "OK K.O.! Let's Be Heroes".to_string(),
            seasons: 3,
        }
    }

    #[wasm_bindgen(getter)]
    /// The name of the series
    pub fn name(&self) -> String {
        self.name.clone()
    }
}
