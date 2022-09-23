use std::fmt::{Display, Formatter};
use egg::{EGraph, Id, Language, LanguageChildren, Pattern};
use derive_more::*;
use petgraph::prelude::*;
use slotmap::{new_key_type, SlotMap};

pub struct Gear {
    header: GearHeader,
    inner: GearInner,
}

impl Gear {
    pub fn run(&self, inputs: Vec<Value>) -> Result<Vec<Value>> {
        //TODO: Are these checks necessary or can this be ensured otherwise?
        self.header.check_input_types(&inputs)?;
        let result = self.inner.run(inputs)?;
        self.header.check_output_types(&result)?;
        Ok(result)
    }
}

pub struct GearHeader {
    name: String,
    inputs: Vec<IOPutHeader>,
    outputs: Vec<IOPutHeader>,
}

impl GearHeader {
    fn check_input_types(&self, inputs: &Vec<Value>) -> Result<()> {
        self.inputs
            .iter()
            .zip(inputs)
            .all(|(header, value)| header.ty == value.ty())
            .then_some(())
            .ok_or(Error::InputTypeMismatch)
    }

    fn check_output_types(&self, outputs: &Vec<Value>) -> Result<()> {
        self.outputs
            .iter()
            .zip(outputs)
            .all(|(header, value)| header.ty == value.ty())
            .then_some(())
            .ok_or(Error::OutputTypeMismatch)
    }
}

#[derive(Debug)]
pub enum Error {
    InputTypeMismatch,
    OutputTypeMismatch,
    Unimplemented,
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct IOPutHeader {
    name: String,
    ty: Type,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Type {
    Float,
    #[allow(dead_code)]
    Unimplemented,
}

#[derive(Clone, Copy, Debug, PartialEq, From, TryInto)]
pub enum Value {
    Float(f32),
    Unimplemented,
}

impl Value {
    fn ty(&self) -> Type {
        match self {
            Value::Float(_) => Type::Float,
            _ => unimplemented!(),
        }
    }
}

new_key_type! {pub struct GearId;}

enum GearInner {
    RuntimeFunction(fn(Vec<Value>) -> Result<Vec<Value>>),
    Composite(Composite),
    #[allow(dead_code)]
    Unimplemented,
}

impl GearInner {
    pub fn run(&self, inputs: Vec<Value>) -> Result<Vec<Value>> {
        match self {
            GearInner::RuntimeFunction(function) => Ok(function(inputs)?),
            GearInner::Composite(composite) => {
                composite.run(inputs)
            }
            GearInner::Unimplemented => Err(Error::Unimplemented),
        }
    }
}

struct Composite {
    gears: SlotMap<GearId, Gear>,
    graph: EGraph<GearLanguage, ()>,
    outputs: Vec<Id>,
}

impl Composite {
    pub fn run(&self, inputs: Vec<Value>) -> Result<Vec<Value>> {
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum GearLanguage {
    Destructure(GearDestructure),
    Expression(GearExpression),
    In(usize),
}

impl Display for GearLanguage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GearLanguage::Destructure(destr) => write!(f, "Destructure({})", destr.index),
            GearLanguage::Expression(expr) => write!(f, "Gear({})", expr.gear.0.as_ffi()),
            GearLanguage::In(i) => write!(f, "In({})", i),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct GearDestructure {
    index: usize,
    child: Id,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct GearExpression {
    gear: GearId,
    children: Vec<Id>,
}

impl Language for GearLanguage {
    fn matches(&self, other: &Self) -> bool {
        self == other
    }

    fn children(&self) -> &[Id] {
        match self {
            GearLanguage::Destructure(destr) => destr.child.as_slice(),
            GearLanguage::Expression(expr) => &expr.children,
            GearLanguage::In(_) => &[],
        }
    }

    fn children_mut(&mut self) -> &mut [Id] {
        match self {
            GearLanguage::Destructure(destr) => destr.child.as_mut_slice(),
            GearLanguage::Expression(expr) => &mut expr.children,
            GearLanguage::In(_) => &mut [],
        }
    }
}

enum IOG {
    In(In),
    Out(Out),
    Gear(GearId),
}

struct In {
    index: usize,
}

struct Out {
    index: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    fn construct_addition_gear() -> Gear {
        Gear {
            header: GearHeader {
                name: String::from("Addition"),
                inputs: vec![
                    IOPutHeader {
                        name: String::from("augend"),
                        ty: Type::Float,
                    },
                    IOPutHeader {
                        name: String::from("addend"),
                        ty: Type::Float,
                    },
                ],
                outputs: vec![IOPutHeader {
                    name: String::from("sum"),
                    ty: Type::Float,
                }],
            },
            inner: GearInner::RuntimeFunction(|inputs| {
                let in0: f32 = inputs[0].try_into().unwrap();
                let in1: f32 = inputs[1].try_into().unwrap();
                Ok(vec![(in0 + in1).into()])
            }),
        }
    }

    #[test]
    fn check_addition_gear() {
        let gear = construct_addition_gear();
        assert!(gear
            .run(vec![Value::Float(1.0), Value::Float(2.0)])
            .unwrap()
            .into_iter()
            .zip(vec![Value::Float(3.0)])
            .all(|(actual, expected)| actual == expected));
    }

    fn construct_double_gear() -> Gear {
        let mut gears = SlotMap::with_key();
        let addition_gear = gears.insert(construct_addition_gear());
        let mut graph = EGraph::<GearLanguage,()>::default();

        let input = graph.add(GearLanguage::In(0));
        let addition = graph.add(GearLanguage::Expression(GearExpression {
            gear: addition_gear,
            children: vec![input, input],
        }));
        let output = graph.add(GearLanguage::Destructure(GearDestructure {
            index: 0,
            child: addition,
        }));

        graph.rebuild();

        graph.dot().to_png("C:/tmp/eggdot.png").unwrap();


        Gear {
            header: GearHeader {
                name: String::from("Double"),
                inputs: vec![
                    IOPutHeader {
                        name: String::from("single"),
                        ty: Type::Float,
                    },
                ],
                outputs: vec![IOPutHeader {
                    name: String::from("doubled"),
                    ty: Type::Float,
                }],
            },
            inner: GearInner::Composite(Composite {
                gears,
                graph,
                outputs: vec![output]
            }),
        }
    }

    #[test]
    fn check_double_gear() {
        let gear = construct_double_gear();
        assert!(gear
            .run(vec![Value::Float(1.0)])
            .unwrap()
            .into_iter()
            .zip(vec![Value::Float(2.0)])
            .all(|(actual, expected)| actual == expected));
    }
}
