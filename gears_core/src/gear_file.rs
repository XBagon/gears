use std::{collections::HashMap, fs, path::Path};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::gear::Gear;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct GearFile {
    meta_data: MetaData,
    gear: Gear,
}

impl GearFile {
    pub fn new(meta_data: MetaData, gear: Gear) -> Self {
        Self { meta_data, gear }
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct MetaData {
    pub name: String,
    pub description: String,
    pub author: String,
    pub tags: HashMap<String, String>,
}

impl GearFile {
    pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<GearFile> {
        let byte_vec = fs::read(path)?;
        let gear_file = postcard::from_bytes(&byte_vec)?;
        Ok(gear_file)
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let byte_vec = postcard::to_stdvec(self)?;
        fs::write(path, byte_vec)?;
        Ok(())
    }
}
