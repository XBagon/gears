use super::*;

mod math;

pub fn init(register: &mut GearRegister) {
    math::init(register);
}

pub struct GearInternal {
    pub function: fn(Vec<TypedValue>) -> Vec<TypedValue>,
}

impl GearInternal {
    pub fn new(function: fn(Vec<TypedValue>) -> Vec<TypedValue>) -> Self {
        Self {
            function,
        }
    }
}

impl Geared for GearInternal {
    fn evaluate(&self, input: Vec<TypedValue>) -> Vec<TypedValue> {
        (self.function)(input)
    }
}


macro_rules! template {
    ($name:ident ($($inname:ident: $inty:ident),*) -> ($($outname:ident: $outty:ident),*) {$code:block}) => {
        struct $name;
        impl $name {
            fn function(input: Vec<TypedValue>) -> Vec<TypedValue> {
                let mut input = input.iter();
                $(let $inname = if let &TypedValue::$inty($inname) = input.next().unwrap() {
                    $inname
                } else { unreachable!() };)*
                drop(input);


                $(
                    #[allow(unused_mut)]
                    let mut $outname;
                )*

                $code

                vec![$(TypedValue::$outty($outname)),*]
            }

            fn template() -> Gear {
                Gear::new(
                    String::from(stringify!($name)),
                    vec![$(IOInformation::new(String::from(stringify!($inname)), TypedValue::$inty(Default::default()).ty())),*],
                    vec![$(IOInformation::new(String::from(stringify!($outname)), TypedValue::$outty(Default::default()).ty())),*],
                    GearImplementation::GearInternal(GearInternal::new(Self::function))
                )
            }
        }
    }
}

pub(crate) use template;