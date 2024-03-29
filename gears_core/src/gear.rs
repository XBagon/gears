use crate::runtime::Runtime;
use crate::*;
use egg::*;
pub use gears_wasm::WasmGear;
use serde::{Deserialize, Serialize};
use slotmap::{new_key_type, SlotMap};
use std::fmt::{Debug, Display, Formatter};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Gear {
    pub header: GearHeader,
    inner: GearInner,
}

impl Gear {
    pub fn new(header: GearHeader, inner: GearInner) -> Gear {
        Gear { header, inner }
    }

    pub fn run(&self, input: Value) -> Result<Value> {
        //TODO: Are these checks necessary or can this be ensured otherwise?
        self.header.check_input_type(&input)?;
        let result = self.inner.run(input)?;
        self.header.check_output_type(&result)?;
        Ok(result)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GearHeader {
    pub name: String,
    pub inputs: Vec<IOPutHeader>,
    pub outputs: Vec<IOPutHeader>,
}

impl GearHeader {
    fn check_input_type(&self, input: &Value) -> Result<()> {
        let input_strct: &Struct = input.to_struct()?;
        self.inputs
            .iter()
            .zip(&input_strct.0)
            .all(|(header, value)| header.ty == value.ty())
            .then_some(())
            .ok_or(Error::InputTypeMismatch)
    }

    fn check_output_type(&self, output: &Value) -> Result<()> {
        let output_strct: &Struct = output.to_struct()?;
        self.outputs
            .iter()
            .zip(&output_strct.0)
            .all(|(header, value)| header.ty == value.ty())
            .then_some(())
            .ok_or(Error::OutputTypeMismatch)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IOPutHeader {
    name: String,
    ty: Type,
}

impl IOPutHeader {
    pub fn new(name: String, ty: Type) -> Self {
        Self { name, ty }
    }
}

new_key_type! {pub struct GearId;}

#[derive(Serialize, Deserialize)]
pub enum GearInner {
    #[serde(skip)]
    RuntimeFunction(fn(Value) -> Result<Value>),
    Composite(Box<CompositeGear>),
    Wasm(WasmGear),
    Reference(GearUuid),
    #[allow(dead_code)]
    Unimplemented,
}

impl GearInner {
    pub fn into_gear(self, header: GearHeader) -> Gear {
        Gear {
            header,
            inner: self,
        }
    }

    pub fn run(&self, input: Value) -> Result<Value> {
        match self {
            GearInner::RuntimeFunction(function) => Ok(function(input)?),
            GearInner::Composite(composite) => composite.run(input),
            GearInner::Reference(_) => todo!(),
            GearInner::Wasm(_) => todo!(),
            GearInner::Unimplemented => Err(Error::Unimplemented),
        }
    }
}

impl Debug for GearInner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RuntimeFunction(_) => f
                .debug_tuple("RuntimeFunction")
                .field(&"<internal>")
                .finish(),
            Self::Composite(arg0) => f.debug_tuple("Composite").field(arg0).finish(),
            Self::Wasm(wasm) => f
                .debug_tuple("Wasm")
                .field(&format!("<{} bytes wasm>", wasm.size()))
                .finish(),
            Self::Reference(uuid) => f
                .debug_tuple("Reference")
                .field(&uuid.0.hyphenated())
                .finish(),
            Self::Unimplemented => write!(f, "Unimplemented"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompositeGear {
    pub gears: SlotMap<GearId, Gear>,
    pub graph: EGraph<GearLanguage, ()>,
    pub outputs: Vec<Id>,
}

impl CompositeGear {
    pub fn run(&self, input: Value) -> Result<Value> {
        let mut runtime = Runtime {
            expr: RecExpr::default(),
            input,
            context: self,
        };

        let rules = Vec::new();
        let runner = Runner::default().with_egraph(self.graph.clone()).run(rules); //TODO: use replace_with instead of clone
        let extractor = Extractor::new(&runner.egraph, AstSize);

        let mut output_vec = Vec::new();
        for &output in &self.outputs {
            let (_best_cost, best_expr) = extractor.find_best(output);
            runtime.expr = best_expr;
            output_vec.push(runtime.run()?);
            //panic!("{:?}", best_expr);
        }
        Ok(output_vec.into())
    }
}

#[derive(Serialize, Deserialize)]
pub struct GearUuid(Uuid);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GearLanguage {
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GearDestructure {
    pub index: usize,
    pub child: Id,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GearExpression {
    pub gear: GearId,
    pub children: Vec<Id>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    macro_rules! assert_gear {
        ($gear:expr, $input:expr, $output:expr) => {{
            use crate::gear::WrapInStruct;
            assert_eq!(
                $gear.run($input.wrap_in_struct().into()).unwrap(),
                $output.wrap_in_struct().into()
            );
        }};
    }

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
            inner: GearInner::RuntimeFunction(|input| {
                let mut inputs = input.into_struct().unwrap().0;
                let in1: f32 = inputs.pop().unwrap().try_into().unwrap();
                let in0: f32 = inputs.pop().unwrap().try_into().unwrap();
                Ok(vec![Value::Float(in0 + in1)].into())
            }),
        }
    }

    #[test]
    fn check_addition_gear() {
        let gear = construct_addition_gear();
        assert_gear!(
            gear,
            vec![Value::Float(1.0), Value::Float(2.0)],
            Value::Float(3.0)
        );
    }

    fn construct_double_gear() -> Gear {
        let mut gears = SlotMap::with_key();
        let addition_gear = gears.insert(construct_addition_gear());
        let mut graph = EGraph::<GearLanguage, ()>::default();

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

        Gear {
            header: GearHeader {
                name: String::from("Double"),
                inputs: vec![IOPutHeader {
                    name: String::from("single"),
                    ty: Type::Float,
                }],
                outputs: vec![IOPutHeader {
                    name: String::from("doubled"),
                    ty: Type::Float,
                }],
            },
            inner: GearInner::Composite(Box::new(CompositeGear {
                gears,
                graph,
                outputs: vec![output],
            })),
        }
    }

    #[test]
    fn check_double_gear() {
        let gear = construct_double_gear();
        assert_gear!(gear, Value::Float(1.0), Value::Float(2.0))
    }
}
