use super::*;
use std::process::{Command, ExitStatus};

pub struct Gears {
    pub generic_command: GearId,
}

impl Gears {
    pub fn init(gears: &mut GearSlotMap) -> Self {
        Self {
            generic_command: gears.insert(GearGenericCommand::template()),
        }
    }
}

pub struct GearGenericCommand;
impl GearGenericCommand {
    pub fn template() -> Gear {
        Gear {
            name: String::from("GenericCommand"),
            inputs: Vec::new(),
            outputs: Vec::new(),
            implementation: GearImplementation::GearGenericCommand(GearGenericCommand),
        }
    }
}

impl Geared for GearGenericCommand {
    fn evaluate(
        &self,
        _register: &GearRegister,
        input: Vec<TypedValue>,
    ) -> Result<Vec<TypedValue>> {
        let mut input = input.into_iter().map(|input| {
            if let TypedValue::String(s) = input {
                s
            } else {
                unreachable!()
            }
        });
        let output = Command::new(input.next().unwrap()).args(input).output()?;
        Ok(vec![
            TypedValue::I32(extract_exit_code(output.status)?),
            TypedValue::String(String::from_utf8(output.stdout)?),
            TypedValue::String(String::from_utf8(output.stderr)?),
        ])
    }
}

pub struct GearCommand {
    program: String,
}

impl GearCommand {
    pub fn new(program: String) -> Self {
        Self { program }
    }
}

impl Geared for GearCommand {
    fn evaluate(
        &self,
        _register: &GearRegister,
        input: Vec<TypedValue>,
    ) -> Result<Vec<TypedValue>> {
        let output = Command::new(&self.program)
            .args(input.into_iter().map(|input| {
                if let TypedValue::String(s) = input {
                    s
                } else {
                    unreachable!()
                }
            }))
            .output()?;
        Ok(vec![
            TypedValue::I32(extract_exit_code(output.status)?),
            TypedValue::String(String::from_utf8(output.stdout)?),
            TypedValue::String(String::from_utf8(output.stderr)?),
        ])
    }
}

#[cfg(target_family = "unix")]
fn extract_exit_code(status: ExitStatus) -> Result<i32> {
    use std::os::unix::process::ExitStatusExt;
    Ok(status
        .code()
        .ok_or_else(|| crate::gear::Error::TerminatedBySignal(status.signal().unwrap()))?)
}

#[cfg(not(target_family = "unix"))]
fn extract_exit_code(status: ExitStatus) -> Result<i32> {
    Ok(status.code().unwrap())
}
