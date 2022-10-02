use std::{
    collections::HashMap,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::{gear::Gear, gear_file};

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
        dbg!(&file_bytes);
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

#[test]
fn ser_de() {
    use crate::gear::*;

    let gear_file = GearFile::new(
        MetaData::new(
            String::from("Test Gear"),
            String::from("This gear will be tried to be serialized and then deserialized again!"),
            String::from("ME"),
            HashMap::from([(String::from("test_tag"), String::from("is important"))]),
        ),
        Gear::new(
            GearHeader {
                name: String::from("test"),
                inputs: vec![
                    IOPutHeader::new(String::from(String::from("some_in")), crate::Type::Float),
                    IOPutHeader::new(
                        String::from(String::from("other_in")),
                        crate::Type::Unimplemented,
                    ),
                ],
                outputs: vec![
                    IOPutHeader::new(
                        String::from(String::from("some_out")),
                        crate::Type::Unimplemented,
                    ),
                    IOPutHeader::new(String::from(String::from("other_out")), crate::Type::Float),
                ],
            },
            GearInner::Unimplemented,
        ),
    );
    let bytes = postcard::to_stdvec(&gear_file).unwrap();
    let mut deserializer = postcard::Deserializer::from_bytes(&bytes);
    let gear_file: GearFile = serde_path_to_error::deserialize(&mut deserializer).unwrap();
    //let gear_file: GearFile = postcard::from_bytes(&bytes).unwrap();
}

#[test]
fn load_add() {
    let gear_file = GearFile::read_from_file("../gearify/tests/output/add.gear").unwrap();
    dbg!(gear_file);
}
