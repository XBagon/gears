use super::*;
use slotmap::{new_key_type, Key, SecondaryMap, SlotMap};
use std::collections::HashMap;

new_key_type! { pub struct GearInstanceId; }

pub struct GearCompound {
    gears: SlotMap<GearInstanceId, GearInstance>,
    connections: HashMap<GearInstanceId, Vec<(GearInstanceId, usize)>>,
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

    pub fn connect(
        &mut self,
        out_gear_id: GearInstanceId,
        out_index: usize,
        in_gear_id: GearInstanceId,
        in_index: usize,
    ) {
        let vec = self.connections.entry(in_gear_id).or_default();
        if vec.len() <= in_index {
            vec.resize(in_index + 1, (GearInstanceId::null(), 0)); //TODO: set up right size when adding gear
        }
        vec[in_index] = (out_gear_id, out_index);
    }

    pub fn evaluate_instance(
        &self,
        register: &GearRegister,
        instance_id: GearInstanceId,
        input: Vec<TypedValue>,
    ) -> Result<Vec<TypedValue>> {
        let gear_id = self.gears.get(instance_id).unwrap().gear;
        register
            .gears
            .get(gear_id)
            .unwrap()
            .evaluate(register, input)
    }
}

impl Geared for GearCompound {
    fn evaluate(&self, register: &GearRegister, input: Vec<TypedValue>) -> Result<Vec<TypedValue>> {
        //Post-Order DFS with cache for evaluations
        let mut stack = Vec::new();
        let mut cache: SecondaryMap<GearInstanceId, Vec<TypedValue>> =
            SecondaryMap::with_capacity(self.gears.len());
        let mut visited: SecondaryMap<GearInstanceId, ()> =
            SecondaryMap::with_capacity(self.gears.len());

        stack.push(self.output_id);
        cache.insert(self.input_id, input);

        while let Some(&current_gear_id) = stack.last() {
            if !visited.contains_key(current_gear_id) {
                //FIXME: somehow fill before? or at least use entry API
                visited.insert(current_gear_id, ());

                if let Some(connections) = self.connections.get(&current_gear_id) {
                    for &(connected_id, _out_index) in connections {
                        if !visited.contains_key(connected_id) {
                            stack.push(connected_id);
                        }
                    }
                }
            } else {
                stack.pop();
                if !cache.contains_key(current_gear_id) {
                    let gear_instance = &self.gears[current_gear_id];
                    let gear_id = gear_instance.gear;
                    let gear = &register.gears[gear_id];

                    let connections = &self.connections[&current_gear_id];
                    let inputs = connections
                        .iter()
                        .map(|&(gear_id, out_index)| cache[gear_id][out_index].clone())
                        .collect();

                    cache.insert(current_gear_id, gear.evaluate(register, inputs)?);
                    //? -> early return/abort
                }
            }
        }

        Ok(cache.remove(self.output_id).unwrap())
    }
}
