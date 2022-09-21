use super::*;
use slotmap::{Key, SecondaryMap};
use std::collections::HashMap;

#[derive(Clone)]
pub struct GearCompound {
    pub connections: HashMap<GearId, Vec<(GearId, usize)>>,
    pub input_id: GearId,
    pub output_id: GearId,
}

impl GearCompound {
    pub fn new(register: &mut GearRegister, num_inputs: usize, num_outputs: usize) -> Self {
        let mut input = register.instantiator(register.templates.special.io.input);
        input.gear.outputs =
            vec![IOInformation::new(String::from("in"), TypedValue::None.ty()); num_inputs];
        let input_id = input.instantiate();

        let mut output = register.instantiator(register.templates.special.io.output);
        output.gear.inputs =
            vec![IOInformation::new(String::from("out"), TypedValue::None.ty()); num_outputs];
        let output_id = output.instantiate();

        Self {
            connections: HashMap::new(),
            input_id,
            output_id,
        }
    }

    pub fn connect(
        &mut self,
        out_gear_id: GearId,
        out_index: usize,
        in_gear_id: GearId,
        in_index: usize,
    ) {
        let vec = self.connections.entry(in_gear_id).or_default();
        if vec.len() <= in_index {
            vec.resize(in_index + 1, (GearId::null(), 0)); //TODO: set up right size when adding gear
        }
        vec[in_index] = (out_gear_id, out_index);
    }

    pub fn evaluate_instance(
        &self,
        register: &GearRegister,
        gear_id: GearId,
        input: Vec<TypedValue>,
    ) -> Result<Vec<TypedValue>> {
        register.gears[gear_id].evaluate(register, input)
    }
}

impl Geared for GearCompound {
    fn evaluate(&self, register: &GearRegister, input: Vec<TypedValue>) -> Result<Vec<TypedValue>> {
        //Post-Order DFS with cache for evaluations
        let mut stack = Vec::new();
        let mut cache: SecondaryMap<GearId, Vec<TypedValue>> = SecondaryMap::new(); //capacity known to serdeble wrapper around GearCompound
        let mut visited: SecondaryMap<GearId, ()> = SecondaryMap::new(); //capacity known to serdeble wrapper around GearCompound

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
                    let gear = &register.gears[current_gear_id];

                    dbg!(&self.connections);
                    let inputs = if let Some(connections) = &self.connections.get(&current_gear_id)
                    {
                        connections
                            .iter()
                            .map(|&(gear_id, out_index)| cache[gear_id][out_index].clone())
                            .collect()
                    } else {
                        Vec::new()
                    };
                    dbg!(&inputs);
                    cache.insert(current_gear_id, gear.evaluate(register, inputs)?);
                    //? -> early return/abort
                }
            }
        }

        Ok(cache.remove(self.output_id).unwrap())
    }
}
