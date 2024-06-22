use log::{debug, info};

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
        water_density: WaterDensity,
    ) -> Self {
        ShipTrimming {
            lcb,
            displacement,
            lcg,
            displacement_tonnage,
            hydrastatic_curves: hydrostatic_curves,
            ship_dimensions,
            water_density,
        }
    }

    fn trim_achieved(
        &self,
        displacement_tonnage: f64,
        current_displacement: f64,
        lcg: f64,
        lcb: f64,
    ) -> bool {
        (displacement_tonnage - self.water_density.water_density() * current_displacement).abs()
            <= 0.004 * displacement_tonnage
            && (lcg - lcb).abs() <= 0.001 * self.ship_dimensions.length_between_perpendiculars()
    }

    pub fn trim(&self) -> Result<(f64, f64), String> {
        let max_draft = self.hydrastatic_curves.max_draft();
        let displacement_tonnage = self.displacement_tonnage.displacement_tonnage();
        let mean_draft = self
            .hydrastatic_curves
            .mean_draft_by_displacement_tonnage(displacement_tonnage)?;
        let area_water_line = self
            .hydrastatic_curves
            .get_data_by_draft(mean_draft, HydrostaticTypeData::WaterlineArea)?;
        let lcg = self.lcg.lcg();

        let displacement = self.displacement.displacement_by_mass(displacement_tonnage);
        let mut lcb = self
            .hydrastatic_curves
            .get_data_by_draft(mean_draft, HydrostaticTypeData::LCB)?;
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
            - ((self.ship_dimensions.length_between_perpendiculars() / 2.0) + lcf)
                * ((lcg - lcb) / r_l);
        let mut current_displacement = self
            .displacement
            .displacement_by_drafts(aft_draft, nose_draft)?;
        lcb = self.lcb.lcb(aft_draft, nose_draft)?;
        let ship_mean_draft = (aft_draft + nose_draft) * 0.5;
        if ship_mean_draft > max_draft {
            return Err(format!("Удифферентовка судна не достигнута из-за превышения максимальной средней осадки судна.\
                                Средняя осадка судна для водоизмещения '{}' м^3 составляет: {} м. Максимально допустимая средняя осадка: {} м.",
                                displacement_tonnage, ship_mean_draft, 13.3));
        }
        if self.trim_achieved(displacement_tonnage, current_displacement, lcg, lcb) {
            info!("Удифферентовка судна на тихой воде достигнута за 1 итерацию.");
            return Ok((aft_draft, nose_draft));
        };
        for i in 2..100 {
            let nose_draft = mean_draft
                + ((displacement - current_displacement) / area_water_line)
                + (self.ship_dimensions.length_between_perpendiculars() / 2.0 - lcf)
                    * ((lcg - lcb) / r_l);

            let aft_draft = mean_draft + ((displacement - current_displacement) / area_water_line)
                - (self.ship_dimensions.length_between_perpendiculars() / 2.0 + lcf)
                    * ((lcg - lcb) / r_l);
            let ship_mean_draft = (aft_draft + nose_draft) * 0.5;
            if ship_mean_draft > max_draft {
                return Err(format!("Удифферентовка судна не достигнута из-за превышения максимальной средней осадки судна.\
                                    Средняя осадка судна для водоизмещения '{}' м^3 составляет: {} м. Максимально допустимая средняя осадка: {} м.",
                                    displacement_tonnage, ship_mean_draft, 13.3));
            }
            current_displacement = self
                .displacement
                .displacement_by_drafts(aft_draft, nose_draft)?;
            let err = (lcg - lcb).abs();
            let displacement_error = (displacement_tonnage
                - self.water_density.water_density() * current_displacement)
                .abs();
            info!("displ_error = {displacement_error}");
            info!("aft_draft = {}, nose_draft = {}", aft_draft, nose_draft);
            info!("'lcg - lcb' = {}", err);
            info!("lcg = {}, lcb = {}", lcg, lcb);
            info!("ship_mean_draft = {ship_mean_draft}");
            info!("displacement = {displacement}, current_disp = {current_displacement}");
            if self.trim_achieved(displacement_tonnage, current_displacement, lcg, lcb) {
                info!(
                    "Удифферентовка судна на тихой воде достигнута за {} итераций.",
                    i
                );
                return Ok((aft_draft, nose_draft));
            };
            lcb = self.lcb.lcb(aft_draft, nose_draft)?;
        }
        let err = (lcg - lcb).abs();
        let displacement_error = (displacement_tonnage
            - self.water_density.water_density() * current_displacement)
            .abs();
        debug!("displ_error = {displacement_error}");
        debug!("aft_draft = {}, nose_draft = {}", aft_draft, nose_draft);
        debug!("'lcg - lcb' = {}", err);
        debug!("lcg = {}, lcb = {}", lcg, lcb);
        debug!("ship_mean_draft = {ship_mean_draft}");
        debug!("displacement = {displacement}, current_disp = {current_displacement}");
        Err("Удифферентовка судна не достигнута из-за превышения максимального количества итераций.".to_string())
    }
}
