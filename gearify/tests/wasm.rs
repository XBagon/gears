use gearify::save_gear_from_wasm_file;
use gears_core::gear_file::MetaData;
use std::collections::HashMap;

#[test]
fn add() {
    let meta_data = MetaData {
        name: String::from("Add"),
        description: String::from("Adds 2 f32 and returns the sum."),
        author: String::from("gears"),
        tags: HashMap::from([
            (String::from("test_output"), String::from("true"))
        ]),
    };
    save_gear_from_wasm_file("tests/output/add.gear", meta_data, "../target/wasm32-wasi/release/examples/add.wasm").unwrap();
}
