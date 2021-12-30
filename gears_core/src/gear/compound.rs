use slotmap::{SlotMap, new_key_type, Key};
use std::collections::HashMap;
use super::*;

new_key_type! { pub struct GearInstanceId; }

pub struct GearCompound {
    gears: SlotMap<GearInstanceId, GearInstance>,
    connections: HashMap<(GearInstanceId, usize),(GearInstanceId, usize)>,
    in_connections: Vec<(GearInstanceId, usize)>,
    out_connections: Vec<(GearInstanceId, usize)>,
}

impl GearCompound {
    fn new(num_inputs: usize, num_outputs: usize) -> Self {
        Self {
            gears: SlotMap::with_key(),
            connections: HashMap::new(),
            in_connections: vec![(GearInstanceId::null(), usize::MAX); num_inputs],
            out_connections: vec![(GearInstanceId::null(), usize::MAX); num_outputs],
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