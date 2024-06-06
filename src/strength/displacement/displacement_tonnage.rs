use crate::strength::{deadweight::deadweight::Deadweight, lightweight::lightweight::Lightweight};

pub(crate) struct DisplacementTonnage {
    lightweight: Lightweight,
    deadweight: Deadweight
}


impl DisplacementTonnage {
    pub fn new(lightweight: Lightweight, deadweight: Deadweight) -> Self {
        DisplacementTonnage { lightweight, deadweight }
    }

    pub fn displacement_tonnage(&self) -> f64 {
        self.lightweight.lightweight() + self.deadweight.deadweight()
    }
}