
use super::spatium_function::SpatiumFunction;

pub struct SpatiumFunctions {
    functions: Vec<SpatiumFunction>
}

impl SpatiumFunctions {
    pub fn new(functions: Vec<SpatiumFunction>) -> Self {
        SpatiumFunctions { functions }
    }

    pub fn filled_zeros(number_spatium: u64, length_spatium: f64) -> Self {
        let mut functions = vec![];
        for id in 0..number_spatium {
            let x1 = id as f64 * length_spatium;
            let x2 = x1 + length_spatium;
            let spatium_function = SpatiumFunction::new(id, x1, x2, 0.0, 0.0);
            functions.push(spatium_function);
        }
        SpatiumFunctions::new(functions)
    }
}