use derive_more::*;
use egg::*;
use slotmap::{new_key_type, SlotMap};
use std::convert::TryInto;
use std::fmt::{Display, Formatter};
use std::ops::Index;

pub struct Gear {
    header: GearHeader,
    inner: GearInner,
}

impl Gear {
    pub fn run(&self, input: Value) -> Result<Value> {
        //TODO: Are these checks necessary or can this be ensured otherwise?
        self.header.check_input_type(&input)?;
        let result = self.inner.run(input)?;
        self.header.check_output_type(&result)?;
        Ok(result)
    }
}

pub struct GearHeader {
    name: String,
    inputs: Vec<IOPutHeader>,
    outputs: Vec<IOPutHeader>,
}

impl GearHeader {
    fn check_input_type(&self, input: &Value) -> Result<()> {
        let input_strct: &Struct = input.to_struct()?;
        self.inputs
            .iter()
            .zip(&*input_strct.0)
            .all(|(header, value)| header.ty == value.ty())
            .then_some(())
            .ok_or(Error::InputTypeMismatch)
    }

    fn check_output_type(&self, output: &Value) -> Result<()> {
        let output_strct: &Struct = output.to_struct()?;
        self.outputs
            .iter()
            .zip(&*output_strct.0)
            .all(|(header, value)| header.ty == value.ty())
            .then_some(())
            .ok_or(Error::OutputTypeMismatch)
    }
}

#[derive(Debug)]
pub enum Error {
    InputTypeMismatch,
    OutputTypeMismatch,
    TriedToDestructureNonStruct(Type),
    Unimplemented,
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct IOPutHeader {
    name: String,
    ty: Type,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Type {
    Float,
    Struct(StructType),
    #[allow(dead_code)]
    Unimplemented,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructType(Vec<Type>);

#[derive(Clone, Debug, PartialEq, From, TryInto)]
#[try_into(ref)]
pub enum Value {
    Float(f32),
    Struct(Struct),
    #[allow(dead_code)]
    Unimplemented,
}

impl Value {
    pub fn from_vec(vec: Vec<Value>) -> Self {
        Self::Struct(vec.into())
    }

    fn ty(&self) -> Type {
        match self {
            Value::Float(_) => Type::Float,
            Value::Struct(strct) => strct.ty(),
            _ => unimplemented!(),
        }
    }

    pub fn to_struct(&self) -> Result<&Struct> {
        self.try_into()
            .map_err(|_| Error::TriedToDestructureNonStruct(self.ty()))
    }

    pub fn into_struct(self) -> Result<Struct> {
        let ty = self.ty();
        self.try_into()
            .map_err(|_| Error::TriedToDestructureNonStruct(ty))
    }
}

impl From<Vec<Value>> for Value {
    fn from(vec: Vec<Value>) -> Self {
        Self::from_vec(vec)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Struct(Vec<Value>);

impl Index<usize> for Struct {
    type Output = Value;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Struct {
    fn ty(&self) -> Type {
        Type::Struct(StructType(self.0.iter().map(|f| f.ty()).collect()))
    }
}

impl From<Vec<Value>> for Struct {
    fn from(vec: Vec<Value>) -> Self {
        Self(vec)
    }
}

trait WrapInStruct {
    fn wrap_in_struct(self) -> Struct;
}

impl WrapInStruct for Vec<Value> {
    fn wrap_in_struct(self) -> Struct {
        self.into()
    }
}

impl WrapInStruct for Value {
    fn wrap_in_struct(self) -> Struct {
        vec![self].into()
    }
}

new_key_type! {pub struct GearId;}

enum GearInner {
    RuntimeFunction(fn(Value) -> Result<Value>),
    Composite(Box<Composite>),
    #[allow(dead_code)]
    Unimplemented,
}

impl GearInner {
    pub fn run(&self, input: Value) -> Result<Value> {
        match self {
            GearInner::RuntimeFunction(function) => Ok(function(input)?),
            GearInner::Composite(composite) => composite.run(input),
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

struct Runtime<'a> {
    context: &'a Composite,
    expr: RecExpr<GearLanguage>,
    input: Value,
}

impl<'a> Runtime<'a> {
    pub fn run(&self) -> Result<Value> {
        let top_node = self.expr.as_ref().last().unwrap();
        self.run_node(top_node)
    }

    fn run_node(&self, current_node: &GearLanguage) -> Result<Value> {
        match current_node {
            GearLanguage::Destructure(destr) => {
                let input = self.run_node(&self.expr[destr.child])?;
                Ok(input.to_struct()?[destr.index].clone())
            }
            GearLanguage::Expression(expr) => {
                let inputs = expr
                    .children
                    .iter()
                    .copied()
                    .flat_map(|c| self.run_node(&self.expr[c]))
                    .collect::<Vec<_>>()
                    .into();
                self.context.gears[expr.gear].run(inputs)
            }
            GearLanguage::In(i) => Ok(self.input.to_struct()?[*i].clone()),
        }
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

        graph.dot().to_png("C:/tmp/eggdot.png").unwrap();

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
            inner: GearInner::Composite(Box::new(Composite {
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
