use std::rc::Rc;

use crate::strength::{deadweight::deadweight::Deadweight, lightweight::lightweight::Lightweight};

pub struct DisplacementTonnage {
    lw: Lightweight,
    dw: Rc<Deadweight>,
}

impl DisplacementTonnage {
    pub fn new(lw: Lightweight, dw: Rc<Deadweight>) -> Self {
        DisplacementTonnage { lw: lw, dw: dw }
    }

    pub fn displacement_tonnage(&self) -> f64 {
        self.lw.lightweight() + self.dw.deadweight()
    }
}
