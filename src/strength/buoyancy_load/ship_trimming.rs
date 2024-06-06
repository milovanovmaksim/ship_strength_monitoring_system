use crate::strength::{bonjean_scale::lcb::LCB, displacement::{displacement::Displacement, displacement_tonnage::DisplacementTonnage}, hydrostatic_curves::{hydrostatic_curves::HydrostaticCurves, hydrostatic_typedata::HydrostaticTypeData}};
use super::lcg::LCG;


pub(crate) struct ShipTrimming<'a> {
    lcb: LCB<'a>,
    displacement: Displacement<'a>,
    lcg: LCG,
    displacement_tonnage: DisplacementTonnage,
    hydrastatic_curves: HydrostaticCurves,
}

impl ShipTrimming<'_> {
    pub fn new(lcb: LCB, displacement: Displacement, lcg: LCG, displacement_tonnage: DisplacementTonnage, hydrastatic_curves: HydrostaticCurves) -> Self {
        ShipTrimming { lcb, displacement, lcg, displacement_tonnage, hydrastatic_curves }
    }

    pub fn trim(&self) -> (f64, f64) {
        let displacement_tonnage = self.displacement_tonnage.displacement_tonnage();
        let mean_draft = self.hydrastatic_curves.mean_draft_by_displacement_tonnage(displacement_tonnage);
        let displacement = self.displacement.displacement_by_mass(displacement_tonnage);
    }


}