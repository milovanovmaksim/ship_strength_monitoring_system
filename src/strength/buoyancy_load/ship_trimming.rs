use super::lcg::LCG;
use crate::strength::{
    bonjean_scale::lcb::LCB,
    displacement::{displacement::Displacement, displacement_tonnage::DisplacementTonnage},
    hydrostatic_curves::{
        hydrostatic_curves::HydrostaticCurves, hydrostatic_typedata::HydrostaticTypeData,
    },
    ship::ship_dimensions::ShipDimensions,
};

pub(crate) struct ShipTrimming<'a> {
    lcb: LCB<'a>,
    displacement: Displacement<'a>,
    lcg: LCG,
    displacement_tonnage: DisplacementTonnage,
    hydrastatic_curves: HydrostaticCurves,
    ship_dimensions: ShipDimensions,
}

impl<'a> ShipTrimming<'a> {
    pub fn new(
        lcb: LCB<'a>,
        displacement: Displacement<'a>,
        lcg: LCG,
        displacement_tonnage: DisplacementTonnage,
        hydrastatic_curves: HydrostaticCurves,
        ship_dimensions: ShipDimensions,
    ) -> Self {
        ShipTrimming {
            lcb,
            displacement,
            lcg,
            displacement_tonnage,
            hydrastatic_curves,
            ship_dimensions,
        }
    }

    pub fn trim(&self) -> Result<(f64, f64), String> {
        let displacement_tonnage = self.displacement_tonnage.displacement_tonnage();
        let mean_draft = self
            .hydrastatic_curves
            .mean_draft_by_displacement_tonnage(displacement_tonnage)?;
        let displacement = self.displacement.displacement_by_mass(displacement_tonnage);
        let lcb = self.lcb.lcb(mean_draft, mean_draft)?;
        let r_l = self.hydrastatic_curves.get_data_by_draft(
            mean_draft,
            HydrostaticTypeData::LongitudinalMetacentricRadius,
        )?;
        todo!();
    }
}
