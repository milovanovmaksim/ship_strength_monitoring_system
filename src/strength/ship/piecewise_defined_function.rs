
use super::spatium_function::SpatiumFunction;

pub struct PiecewiseDefinedFunction {
    functions: Vec<SpatiumFunction>
}

impl PiecewiseDefinedFunction {
    pub fn new(functions: Vec<SpatiumFunction>) -> Self {
        PiecewiseDefinedFunction { functions }
    }

    pub fn filled_zeros(number_spatium: u64, length_spatium: f64) -> Self {
        let mut functions = vec![];
        for id in 0..number_spatium {
            let x1 = id as f64 * length_spatium;
            let x2 = x1 + length_spatium;
            let spatium_function = SpatiumFunction::new(x1, x2, 0.0, 0.0);
            functions.push(spatium_function);
        }
        PiecewiseDefinedFunction::new(functions)
    }
}