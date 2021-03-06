use super::*;

pub struct Gears {
    pub add: TemplateGearId,
    pub sub: TemplateGearId,
    pub mul: TemplateGearId,
    pub div: TemplateGearId,
}

impl Gears {
    pub fn init(template_gears: &mut TemplateGearMap) -> Self {
        Self {
            add: template_gears.insert(Add::template()),
            sub: template_gears.insert(Sub::template()),
            mul: template_gears.insert(Mul::template()),
            div: template_gears.insert(Div::template()),
        }
    }
}

//TODO: define for other num types

template!(Add (summand1: U64, summand2: U64) -> (sum: U64) {{
    sum = summand1 + summand2
}});

template!(Sub (minuend: U64, subtrahend: U64) -> (difference: U64) {{
    difference = minuend - subtrahend
}});

template!(Mul (factor1: U64, factor2: U64) -> (product: U64) {{
    product = factor1 * factor2
}});

template!(Div (dividend: U64, divisor: U64) -> (fraction: U64) {{
    fraction = dividend / divisor
}});
