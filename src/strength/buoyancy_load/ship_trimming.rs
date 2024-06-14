use log::info;

use super::lcg::LCG;
use crate::{
    core::physical_constants,
    strength::{
        bonjean_scale::{bonjean_scale::BonjeanScale, lcb::LCB},
        displacement::{displacement::Displacement, displacement_tonnage::DisplacementTonnage},
        hydrostatic_curves::{
            hydrostatic_curves::HydrostaticCurves, hydrostatic_typedata::HydrostaticTypeData,
        },
        ship::ship_dimensions::ShipDimensions,
    },
};

pub(crate) struct ShipTrimming<'a> {
    lcb: LCB<'a>,
    displacement: Displacement<'a>,
    lcg: LCG,
    displacement_tonnage: DisplacementTonnage,
    hydrastatic_curves: HydrostaticCurves,
    ship_dimensions: ShipDimensions,
    bonjean_scale: &'a BonjeanScale,
}

impl<'a> ShipTrimming<'a> {
    pub fn new(
        lcb: LCB<'a>,
        displacement: Displacement<'a>,
        lcg: LCG,
        displacement_tonnage: DisplacementTonnage,
        hydrastatic_curves: HydrostaticCurves,
        ship_dimensions: ShipDimensions,
        bonjean_scale: &'a BonjeanScale,
    ) -> Self {
        ShipTrimming {
            lcb,
            displacement,
            lcg,
            displacement_tonnage,
            hydrastatic_curves,
            ship_dimensions,
            bonjean_scale,
        }
    }

    pub fn trim(&self) -> Result<(f64, f64), String> {
        let displacement_tonnage = self.displacement_tonnage.displacement_tonnage();
        let mean_draft = self
            .hydrastatic_curves
            .mean_draft_by_displacement_tonnage(displacement_tonnage)?;
        let displacement = self.displacement.displacement_by_mass(displacement_tonnage);
        let lcg = self.lcg.lcg();
        let lcb = self.lcb.lcb(mean_draft, mean_draft)?;
        let r_l = self.hydrastatic_curves.get_data_by_draft(
            mean_draft,
            HydrostaticTypeData::LongitudinalMetacentricRadius,
        )?;
        let lcf = self
            .hydrastatic_curves
            .get_data_by_draft(mean_draft, HydrostaticTypeData::LCF)?;
        let nose_draft = mean_draft
            + (self.ship_dimensions.length_between_perpendiculars() / 2.0 - lcf)
                * ((lcg - lcb) / r_l);
        let aft_draft = mean_draft
            - (self.ship_dimensions.length_between_perpendiculars() / 2.0 - lcf)
                * ((lcg - lcb) / r_l);
        let current_displacement = self
            .displacement
            .displacement_by_drafts(aft_draft, nose_draft)?;
        let lcb = self.lcb.lcb(aft_draft, nose_draft)?;
        if displacement_tonnage - 1.0 * physical_constants::EART_GRAVITY * current_displacement
            <= 0.004 * displacement_tonnage
            && lcg - lcb <= 0.001 * self.ship_dimensions.length_between_perpendiculars()
        {
            return Ok((aft_draft, nose_draft));
        };
        for i in 0..100 {
            let area_water_line = self.bonjean_scale.area_water_line(aft_draft, nose_draft)?;
            let lcb = self.lcb.lcb(aft_draft, nose_draft)?;
            let nose_draft = mean_draft
                + ((displacement - current_displacement) / area_water_line)
                + (self.ship_dimensions.length_between_perpendiculars() / 2.0 - lcf)
                    * ((lcg - lcb) / r_l);

            let aft_draft = mean_draft + ((displacement - current_displacement) / area_water_line)
                - (self.ship_dimensions.length_between_perpendiculars() / 2.0 + lcf)
                    * ((lcg - lcb) / r_l);
            let current_displacement = self
                .displacement
                .displacement_by_drafts(aft_draft, nose_draft)?;
            let lcb = self.lcb.lcb(aft_draft, nose_draft)?;
            if displacement_tonnage - 1.0 * physical_constants::EART_GRAVITY * current_displacement
                <= 0.004 * displacement_tonnage
                && lcg - lcb <= 0.001 * self.ship_dimensions.length_between_perpendiculars()
            {
                info!(
                    "Удифферентовка судна на тихой воде осуществлена за {} итераций.",
                    i
                );
                return Ok((aft_draft, nose_draft));
            };
        }
        Err("При удиферентовки судна количество итераций превысило максимально допустимое значение в 100 итераций".to_string())
    }
}
