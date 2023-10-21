use super::spatium_function::SpatiumFunction;

pub struct PiecewiseDefinedFunction {
    functions: Vec<SpatiumFunction>
}

impl PiecewiseDefinedFunction {
    pub fn new(functions: Vec<SpatiumFunction>) -> Self {
        PiecewiseDefinedFunction { functions }
    }

}