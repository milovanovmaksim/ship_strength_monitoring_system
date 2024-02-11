use crate::strength::ship::{spatium_function::SpatiumFunction, spatium_functions::SpatiumFunctions};
use super::frame::Frame;

///
/// Масштаба Бонжана судна.
/// Parameters:
///     spatium_functions:  - данные масштаба Бонжана для всех шпаций судна.
#[derive(Debug)]
pub(crate) struct BonjeanScale {
    spatium_functions: SpatiumFunctions
}

impl BonjeanScale {
    pub fn new(spatium_functions: SpatiumFunctions) -> Self {
        BonjeanScale { spatium_functions }
    }

    pub fn frame_by_coodinate(&self, x_coordinate: f64) -> &SpatiumFunction {
        todo!("Вернуть &SpatiumFunction с координатой x. Если такого нет, вернуть ближайший шпангоут")


    }

    pub fn frame_by_id(&self, id : usize) -> Option<&SpatiumFunction> {
        self.spatium_functions.spatium_function_by_id(id)
    }
}

impl IntoIterator for BonjeanScale {
    type Item = SpatiumFunction;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.spatium_functions.into_iter()
    }
}

impl AsRef<Vec<SpatiumFunction>> for BonjeanScale {

    fn as_ref(&self) -> &Vec<SpatiumFunction> {
        &self.spatium_functions.as_ref()
    }
}