use serde::Deserialize;

use crate::core::round::Round;

use super::spatium_function::SpatiumFunction;

#[derive(Debug, PartialEq, Deserialize)]
pub struct SpatiumFunctions {
    spatium_functions: Vec<SpatiumFunction>,
}

impl SpatiumFunctions {
    pub fn new(functions: Vec<SpatiumFunction>) -> Self {
        SpatiumFunctions {
            spatium_functions: functions,
        }
    }

    pub fn filled_zeros(number_spatiums: u64, lbp: f64) -> Self {
        let mut functions = vec![];
        let length_spatium = lbp / number_spatiums as f64;
        let mut start_coordinate = -lbp / 2.0;
        for id in 0..number_spatiums {
            let end_coordinate = start_coordinate + length_spatium;
            let spatium_function = SpatiumFunction::new(
                id,
                start_coordinate.my_round(2),
                end_coordinate.my_round(2),
                0.0,
                0.0,
            );
            functions.push(spatium_function);
            start_coordinate += length_spatium;
        }
        SpatiumFunctions::new(functions)
    }

    pub fn last(&self) -> Option<&SpatiumFunction> {
        self.spatium_functions.last()
    }

    pub fn add(&mut self, term: SpatiumFunction) {
        let id = term.id() as usize;
        if let Some(spatium_function) = self.spatium_functions.get_mut(id) {
            let new_spatium_function = spatium_function.add(term);
            *spatium_function = new_spatium_function;
        }
    }

    pub fn get(&self, id: u64) -> Option<&SpatiumFunction> {
        self.spatium_functions.get(id as usize)
    }
}

impl IntoIterator for SpatiumFunctions {
    type Item = SpatiumFunction;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.spatium_functions.into_iter()
    }
}

impl AsRef<Vec<SpatiumFunction>> for SpatiumFunctions {
    fn as_ref(&self) -> &Vec<SpatiumFunction> {
        &self.spatium_functions
    }
}
