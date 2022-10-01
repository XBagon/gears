use crate::{
    gear::{CompositeGear, GearLanguage},
    *,
};
use egg::RecExpr;

pub struct Runtime<'a> {
    pub context: &'a CompositeGear,
    pub expr: RecExpr<GearLanguage>,
    pub input: Value,
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
