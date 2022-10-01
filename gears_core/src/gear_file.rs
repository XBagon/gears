use std::{
    collections::HashMap,
    fs,
    io::{Read, Write},
    path::Path,
};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::gear::Gear;

const FILE_SIGNATURE: [u8; 8] = *b"\x1F*gears*";
const CURRENT_VERSION: u32 = 0;

#[derive(Serialize, Deserialize, Debug)]
pub struct GearFile {
    meta_data: MetaData,
    gear: Gear,
}

impl GearFile {
    pub fn new(meta_data: MetaData, gear: Gear) -> Self {
        Self { meta_data, gear }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetaData {
    version: u32,
    name: String,
    description: String,
    author: String,
    tags: HashMap<String, String>,
}

impl MetaData {
    pub fn new(
        name: String,
        description: String,
        author: String,
        tags: HashMap<String, String>,
    ) -> Self {
        Self {
            version: CURRENT_VERSION,
            name,
            description,
            author,
            tags,
        }
    }
}

impl GearFile {
    pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<GearFile> {
        let mut file = fs::File::open(path)?;

        let mut file_signature = [0u8; 8];
        file.read_exact(&mut file_signature)?;
        if file_signature != FILE_SIGNATURE {
            return Err(anyhow!("Invalid file signature!"));
        }

        let mut file_bytes = Vec::new();
        file.read_to_end(&mut file_bytes)?;
        let gear_file = postcard::from_bytes(&file_bytes)?;
        Ok(gear_file)
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file_bytes = postcard::to_stdvec(self)?;

        let mut file = fs::File::create(path)?;

        file.write_all(&FILE_SIGNATURE)?;

        file.write_all(&file_bytes)?;
        Ok(())
    }
}
