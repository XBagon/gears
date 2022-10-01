use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Serialize, Deserialize)]
pub struct WasmGear {
    wasm: Vec<u8>,
}

impl WasmGear {
    pub fn from_wasm(wasm: Vec<u8>) -> WasmGear {
        WasmGear { wasm }
    }

    pub fn from_wasm_file<P: AsRef<Path>>(path: P) -> Result<WasmGear> {
        let wasm = fs::read(path)?;
        Ok(WasmGear { wasm })
    }

    pub fn size(&self) -> usize {
        self.wasm.len()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn compiles() {}
}
