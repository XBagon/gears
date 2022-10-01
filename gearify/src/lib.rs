use anyhow::{anyhow, Result};
use gears_core::{
    gear::{Gear, GearHeader, GearInner, IOPutHeader, WasmGear},
    gear_file::{GearFile, MetaData},
};
use std::{fs, path::Path};
use wasmtime::{Engine, Module};

pub fn save_gear_from_wasm_file<P: AsRef<Path>>(
    gear_path: P,
    meta_data: MetaData,
    wasm_path: P,
) -> Result<()> {
    let gear = from_wasm_file(wasm_path)?;
    let gear_file = GearFile::new(meta_data, gear);

    //Debugging size:
    println!("{:#?}", gear_file);

    gear_file.save_to_file(gear_path)?;
    Ok(())
}

//TODO: proper Error types
fn from_wasm_file<P: AsRef<Path>>(path: P) -> Result<Gear> {
    let wasm = fs::read(&path)?;
    let module = Module::new(&Engine::default(), &wasm)?;
    let module_exports: Vec<_> = module.exports().collect();

    let mut first_function = Option::None;
    for module_export in module_exports {
        match module_export.ty() {
            wasmtime::ExternType::Func(func_ty) => {
                if module_export.name() == "_start" {
                    //skip wasm start function
                    continue;
                }
                if first_function.is_none() {
                    first_function = Some((module_export.name(), func_ty));
                } else {
                    todo!("Warning: Multiple functions exported!");
                }
            }
            _ => {}
        }
    }
    let (name, ty) = if let Some(first_function) = first_function {
        first_function
    } else {
        return Err(anyhow!("Wasm file exports no function!"));
    };
    let inputs = ty
        .params()
        .map(|param| {
            let ty = match param {
                wasmtime::ValType::I32 => todo!(),
                wasmtime::ValType::I64 => todo!(),
                wasmtime::ValType::F32 => gears_core::Type::Float,
                wasmtime::ValType::F64 => todo!(),
                wasmtime::ValType::V128 => todo!(),
                wasmtime::ValType::FuncRef => todo!(),
                wasmtime::ValType::ExternRef => todo!(),
            };
            IOPutHeader::new(String::new(), ty)
        })
        .collect();

    let outputs = ty
        .results()
        .map(|param| {
            let ty = match param {
                wasmtime::ValType::I32 => todo!(),
                wasmtime::ValType::I64 => todo!(),
                wasmtime::ValType::F32 => gears_core::Type::Float,
                wasmtime::ValType::F64 => todo!(),
                wasmtime::ValType::V128 => todo!(),
                wasmtime::ValType::FuncRef => todo!(),
                wasmtime::ValType::ExternRef => todo!(),
            };
            IOPutHeader::new(String::new(), ty)
        })
        .collect();
    let header = GearHeader {
        name: name.to_owned(),
        inputs,
        outputs,
    };
    let wasm_gear = WasmGear::from_wasm_file(path)?;
    let gear = Gear::new(header, GearInner::Wasm(wasm_gear));
    Ok(gear)
}
