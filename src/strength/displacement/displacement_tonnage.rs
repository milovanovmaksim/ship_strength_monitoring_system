use crate::strength::{deadweight::deadweight::Deadweight, lightweight::lightweight::Lightweight};

pub(crate) struct DisplacementTonnage<'a> {
    lw: Lightweight,
    dw: &'a Deadweight<'a>,
}

impl<'a> DisplacementTonnage<'a> {
    pub fn new(lw: Lightweight, dw: &'a Deadweight<'a>) -> Self {
        DisplacementTonnage {
            lw: lw,
            dw: dw,
        }
    }

    pub fn displacement_tonnage(&self) -> f64 {
        self.lw.lightweight() + self.dw.deadweight()
    }
}
