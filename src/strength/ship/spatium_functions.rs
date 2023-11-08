use super::spatium_function::SpatiumFunction;

#[derive(Debug)]
pub struct SpatiumFunctions {
    functions: Vec<SpatiumFunction>
}

impl SpatiumFunctions {
    pub fn new(functions: Vec<SpatiumFunction>) -> Self {
        SpatiumFunctions { functions }
    }

    pub fn filled_zeros(number_spatiums: u64, length_spatium: f64, length_between_perpendiculars: f64) -> Self {
        let mut functions = vec![];
        let mut start_coordinate = -length_between_perpendiculars / 2.0;
        for id in 0..number_spatiums {
            let end_coordinate = start_coordinate + length_spatium;
            let spatium_function = SpatiumFunction::new(id, start_coordinate, end_coordinate, 0.0, 0.0);
            functions.push(spatium_function);
            start_coordinate += length_spatium;
        }
        SpatiumFunctions::new(functions)
    }

    pub fn add_spatium_function(&mut self, term: &SpatiumFunction) {
        let id = term.id() as usize;
        if let Some(spatium_function) =  self.functions.get_mut(id) {
            let new_spatium_function = spatium_function.add(term);
            *spatium_function = new_spatium_function;
        }
    }

    pub fn functions(&self) -> &Vec<SpatiumFunction> {
        &self.functions
    }
}