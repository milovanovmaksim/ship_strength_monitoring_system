
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

    pub fn add_spatium_function(&mut self, term: SpatiumFunction) {
        let id = term.id() as usize;
        let x1 = term.x1();
        let x2 = term.x2();
        if let Some(spatium_function_old) =  self.functions.get(id) {
            let f_x1 = spatium_function_old.f_x1() + term.f_x1();
            let f_x2 = spatium_function_old.f_x2() + term.f_x2();
            let new_spatium_function = SpatiumFunction::new(id as u64, x1, x2, f_x1, f_x2);
            self.functions.insert(id, new_spatium_function);
        }
    }
}