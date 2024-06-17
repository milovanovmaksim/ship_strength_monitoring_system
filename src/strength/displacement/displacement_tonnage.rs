use crate::strength::{deadweight::deadweight::Deadweight, lightweight::lightweight::Lightweight};

pub(crate) struct DisplacementTonnage<'a> {
    lightweight: Lightweight,
    deadweight: Deadweight<'a>,
}

impl<'a> DisplacementTonnage<'a> {
    pub fn new(lightweight: Lightweight, deadweight: Deadweight<'a>) -> Self {
        DisplacementTonnage {
            lightweight,
            deadweight,
        }
    }

    pub fn displacement_tonnage(&self) -> f64 {
        self.lightweight.lightweight() + self.deadweight.deadweight()
    }
}
