use slotmap::{SlotMap, new_key_type};
use std::collections::HashMap;
use super::*;

new_key_type! { pub struct GearInstanceId; }

pub struct GearCompound {
    gears: SlotMap<GearInstanceId, GearInstance>,
    connections: HashMap<(GearInstanceId, usize),(GearInstanceId, usize)>,
}

impl Geared for GearCompound {
    fn evaluate(&self, input: Vec<TypedValue>) -> Vec<TypedValue> {
        unimplemented!()
    }
}