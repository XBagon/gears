use slotmap::{SlotMap, new_key_type};
use std::collections::HashMap;
use super::*;

new_key_type! { pub struct GearInstanceId; }

pub struct GearCompound {
    gears: SlotMap<GearInstanceId, GearInstance>,
    connections: HashMap<(GearInstanceId, usize),(GearInstanceId, usize)>,
}

impl GearCompound {
    fn new(register: &mut GearRegister, num_inputs: usize, num_outputs: usize) -> Self {
        let mut gears = SlotMap::with_key();
        gears.insert(register.special.io.input.into());
        gears.insert(register.special.io.output.into());
        Self {
            gears,
            connections: HashMap::new(),
        }
    }

    fn add_gear(&mut self, gear_instance: GearInstance) -> GearInstanceId {
        self.gears.insert(gear_instance)
    }

    fn connect(&mut self, in_gear_id: GearInstanceId, in_index: usize, out_gear_id: GearInstanceId, out_index: usize) {
        self.connections.insert((in_gear_id, in_index), (out_gear_id, out_index));
    }
}

impl Geared for GearCompound {
    fn evaluate(&self, input: Vec<TypedValue>) -> Vec<TypedValue> {
        unimplemented!()
    }
}