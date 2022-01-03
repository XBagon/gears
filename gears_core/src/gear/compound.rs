use slotmap::{SlotMap, new_key_type};
use std::collections::HashMap;
use super::*;

new_key_type! { pub struct GearInstanceId; }

pub struct GearCompound {
    gears: SlotMap<GearInstanceId, GearInstance>,
    connections: HashMap<(GearInstanceId, usize),(GearInstanceId, usize)>,
    pub input_id: GearInstanceId,
    pub output_id: GearInstanceId,
}

impl GearCompound {
    pub fn new(register: &GearRegister, num_inputs: usize, num_outputs: usize) -> Self {
        let mut gears = SlotMap::with_key();

        let mut input = register.special.io.input.instance();
        input.output_names = vec![Some(String::from("in")); num_inputs];
        let input_id = gears.insert(input);

        let mut output = register.special.io.output.instance();
        output.input_names = vec![Some(String::from("out")); num_outputs];
        let output_id = gears.insert(output);
        Self {
            gears,
            connections: HashMap::new(),
            input_id,
            output_id,
        }
    }

    pub fn add_gear(&mut self, gear_instance: GearInstance) -> GearInstanceId {
        self.gears.insert(gear_instance)
    }

    pub fn connect(&mut self, out_gear_id: GearInstanceId, out_index: usize, in_gear_id: GearInstanceId, in_index: usize) {
        self.connections.insert((in_gear_id, in_index), (out_gear_id, out_index));
    }

    pub fn evaluate_instance(&self, register: &GearRegister, instance_id: GearInstanceId, input: Vec<TypedValue>) -> Result<Vec<TypedValue>> {
        let gear_id = self.gears.get(instance_id).unwrap().gear;
        register.gears.get(gear_id).unwrap().evaluate(register, input)
    }
}

impl Geared for GearCompound {
    fn evaluate(&self, register: &GearRegister, input: Vec<TypedValue>) -> Result<Vec<TypedValue>> {
        //TODO: implement search through GearCompound to evaluate
        todo!()
    }
}