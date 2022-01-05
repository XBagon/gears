use super::*;

mod math;

pub struct Gears {
    pub math_gears: math::Gears,
}

impl Gears {
    pub fn init(gears: &mut GearSlotMap) -> Self {
        Self {
            math_gears: math::Gears::init(gears),
        }
    }
}

pub struct GearInternal {
    pub function:
        fn(Vec<TypedValue>) -> std::result::Result<Vec<TypedValue>, Box<dyn std::error::Error>>,
}

impl GearInternal {
    pub fn new(
        function: fn(
            Vec<TypedValue>,
        ) -> std::result::Result<Vec<TypedValue>, Box<dyn std::error::Error>>,
    ) -> Self {
        Self { function }
    }
}

impl Geared for GearInternal {
    fn evaluate(
        &self,
        _register: &GearRegister,
        input: Vec<TypedValue>,
    ) -> Result<Vec<TypedValue>> {
        Ok((self.function)(input)?)
    }
}

macro_rules! template {
    ($name:ident ($($inname:ident: $inty:ident),*) -> ($($outname:ident: $outty:ident),*) {$code:block}) => {
        struct $name;
        impl $name {
            fn function(input: Vec<TypedValue>) -> std::result::Result<Vec<TypedValue>, Box<dyn std::error::Error>> {
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

                Ok(vec![$(TypedValue::$outty($outname)),*])
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
