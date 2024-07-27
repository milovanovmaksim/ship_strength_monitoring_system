use crate::strength::{deadweight::deadweight::Deadweight, lightweight::lightweight::Lightweight};

#[derive(Clone, Copy)]
pub struct DisplacementTonnage {
    lw: Lightweight,
    dw: Deadweight,
}

impl DisplacementTonnage {
    pub fn new(lw: Lightweight, dw: Deadweight) -> Self {
        DisplacementTonnage { lw: lw, dw: dw }
    }

    pub fn displacement_tonnage(&self) -> f64 {
        self.lw.lightweight() + self.dw.deadweight()
    }
}
