use log::info;

use super::lcg::LCG;
use crate::{
    core::{physical_constants, water_density::WaterDensity},
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
    lcg: LCG<'a>,
    displacement_tonnage: DisplacementTonnage<'a>,
    hydrastatic_curves: HydrostaticCurves,
    ship_dimensions: ShipDimensions,
    bonjean_scale: &'a BonjeanScale,
    water_density: WaterDensity,
}

impl<'a> ShipTrimming<'a> {
    pub fn new(
        lcb: LCB<'a>,
        displacement: Displacement<'a>,
        lcg: LCG<'a>,
        displacement_tonnage: DisplacementTonnage<'a>,
        hydrostatic_curves: HydrostaticCurves,
        ship_dimensions: ShipDimensions,
        bonjean_scale: &'a BonjeanScale,
        water_density: WaterDensity,
    ) -> Self {
        ShipTrimming {
            lcb,
            displacement,
            lcg,
            displacement_tonnage,
            hydrastatic_curves: hydrostatic_curves,
            ship_dimensions,
            bonjean_scale,
            water_density,
        }
    }

    pub fn trim(&self) -> Result<(f64, f64), String> {
        let max_draft = self.hydrastatic_curves.max_draft();
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
            + ((self.ship_dimensions.length_between_perpendiculars() / 2.0) - lcf)
                * ((lcg - lcb) / r_l);
        let aft_draft = mean_draft
            - ((self.ship_dimensions.length_between_perpendiculars() / 2.0) - lcf)
                * ((lcg - lcb) / r_l);
        let current_displacement = self
            .displacement
            .displacement_by_drafts(aft_draft, nose_draft)?;
        let lcb = self.lcb.lcb(aft_draft, nose_draft)?;
        let ship_mean_draft = (aft_draft + nose_draft) * 0.5;
        if ship_mean_draft * 0.5 > max_draft {
            return Err(format!("Удифферентовка судна не достигнута из-за превышения максимальной средней осадки судна.\
            Средняя осадка судна для данной схемы загрузки = {} м.", ship_mean_draft));
        }
        if displacement_tonnage
            - self.water_density.water_density()
                * physical_constants::EART_GRAVITY
                * current_displacement
            <= 0.004 * displacement_tonnage
            && (lcg - lcb).abs() <= 0.001 * self.ship_dimensions.length_between_perpendiculars()
        {
            info!("Удифферентовка судна на тихой воде достигнута за 1 итерацию.");
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
            let ship_mean_draft = (aft_draft + nose_draft) * 0.5;
            if ship_mean_draft * 0.5 > max_draft {
                return Err(format!("Удифферентовка судна не достигнута из-за превышения максимальной средней осадки судна.\
                Средняя осадка судна для данной схемы загрузки = {} м.", ship_mean_draft));
            }
            let current_displacement = self
                .displacement
                .displacement_by_drafts(aft_draft, nose_draft)?;
            let lcb = self.lcb.lcb(aft_draft, nose_draft)?;
            if displacement_tonnage
                - self.water_density.water_density()
                    * physical_constants::EART_GRAVITY
                    * current_displacement
                <= 0.004 * displacement_tonnage
                && (lcg - lcb).abs() <= 0.001 * self.ship_dimensions.length_between_perpendiculars()
            {
                info!(
                    "Удифферентовка судна на тихой воде достигнута за {} итераций.",
                    i
                );
                return Ok((aft_draft, nose_draft));
            };
        }
        Err("Удифферентовка судна не достигнута из-за превышения максимального количества итераций.".to_string())
    }
}
