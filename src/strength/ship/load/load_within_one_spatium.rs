use crate::strength::ship::spatium_function::SpatiumFunction;

use super::load::Load;

struct LoadWithinOneSpatium {
    load: Load
}

impl LoadWithinOneSpatium {
    pub fn new(load: Load) -> Self {
        LoadWithinOneSpatium { load }
    }

    pub fn intensity_load() -> Vec<SpatiumFunction> {
        todo!();
    }
}