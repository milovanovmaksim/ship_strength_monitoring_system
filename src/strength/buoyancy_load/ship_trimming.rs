use log::info;

use super::lcg::LCG;
use crate::{
    core::water_density::WaterDensity,
    strength::{
        bonjean_scale::lcb::LCB,
        displacement::{displacement::Displacement, displacement_tonnage::DisplacementTonnage},
        hydrostatic_curves::{
            hydrostatic_curves::HydrostaticCurves, hydrostatic_typedata::HydrostaticTypeData,
        },
        ship::ship_dimensions::ShipDimensions,
    },
};

///
/// Удифферентовка судна на тихой воде.
/// Parameters:
pub(crate) struct ShipTrimming<'a> {
    lcb: LCB<'a>,
    displacement: Displacement<'a>,
    lcg: LCG<'a>,
    displacement_tonnage: DisplacementTonnage<'a>,
    hydrostatic_curves: HydrostaticCurves,
    ship_dimensions: ShipDimensions,
    water_density: WaterDensity,
}

impl<'a> ShipTrimming<'a> {
    ///
    /// Основной конструктор.
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
            hydrostatic_curves,
            ship_dimensions,
            water_density,
        }
    }

    ///
    /// Проверяет достигнута ли удифферентовка судна.
    /// Parameters:
    ///     displacement_tonnage - весовое водоизмещение судна при текущей схеме загрузки[т];
    ///     current_displacement - расчетное объемное водоизмещение судна [м^3].
    fn trim_achieved(
        &self,
        displacement_tonnage: f64,
        current_displacement: f64,
        lcg: f64,
        lcb: f64,
    ) -> bool {
        (displacement_tonnage - self.water_density.water_density() * current_displacement).abs()
            <= 0.004 * displacement_tonnage
            && (lcg - lcb).abs() <= 0.001 * self.ship_dimensions.lbp()
    }

    ///
    /// Возвращает валидные осадки кормы и носа судна.
    /// Осадки не должны выходить за диапазон осадки судна указанный в гидростатических кривых.
    fn drafts(&self, mut aft_draft: f64, mut nose_draft: f64) -> (f64, f64) {
        if aft_draft > self.hydrostatic_curves.max_draft() {
            aft_draft = self.hydrostatic_curves.max_draft();
        } else if aft_draft < self.hydrostatic_curves.min_draft() {
            aft_draft = self.hydrostatic_curves.min_draft();
        }
        if nose_draft > self.hydrostatic_curves.max_draft() {
            nose_draft = self.hydrostatic_curves.max_draft();
        } else if nose_draft < self.hydrostatic_curves.min_draft() {
            nose_draft = self.hydrostatic_curves.min_draft();
        }

        (aft_draft, nose_draft)
    }

    ///
    /// Удифферентовка судна методом последовательных приближений.
    /// Возвращает осадку кормы и носа судна (aft_draft, nose_draft).
    pub fn trim(&self) -> Result<(f64, f64), String> {
        let displacement_tonnage = self.displacement_tonnage.displacement_tonnage();
        if displacement_tonnage > self.hydrostatic_curves.max_displacement_tonnage() {
            return Err(format!("Весовое водоизмещение {displacement_tonnage} тонн превысило весовое водоизмещение судна в грузу."));
        }
        let mean_draft = self
            .hydrostatic_curves
            .mean_draft(displacement_tonnage)?
            .unwrap();
        let area_water_line = self
            .hydrostatic_curves
            .get_data_by_draft(mean_draft, HydrostaticTypeData::WaterlineArea)?
            .unwrap();
        let lcg = self.lcg.lcg();
        let lbp = self.ship_dimensions.lbp();
        let displacement = self.displacement.displacement_by_mass(displacement_tonnage);
        let mut lcb = self
            .hydrostatic_curves
            .get_data_by_draft(mean_draft, HydrostaticTypeData::LCB)?
            .unwrap();
        let lmr = self
            .hydrostatic_curves
            .get_data_by_draft(mean_draft, HydrostaticTypeData::LMR)?
            .unwrap();
        let lcf = self
            .hydrostatic_curves
            .get_data_by_draft(mean_draft, HydrostaticTypeData::LCF)?
            .unwrap();
        let mut nose_draft = mean_draft + ((lbp / 2.0) - lcf) * ((lcg - lcb) / lmr);
        let mut aft_draft = mean_draft - ((lbp / 2.0) + lcf) * ((lcg - lcb) / lmr);
        (nose_draft, aft_draft) = self.drafts(aft_draft, nose_draft);
        let mut calculated_displacement = self
            .displacement
            .displacement_by_drafts(aft_draft, nose_draft)?;
        lcb = self.lcb.lcb(aft_draft, nose_draft)?;
        if self.trim_achieved(displacement_tonnage, calculated_displacement, lcg, lcb) {
            info!("Удифферентовка судна на тихой воде достигнута за 1 итерацию.");
            return Ok((aft_draft, nose_draft));
        };
        for i in 2..102 {
            let mut nose_draft = mean_draft
                + ((displacement - calculated_displacement) / area_water_line)
                + (lbp / 2.0 - lcf) * ((lcg - lcb) / lmr);

            let mut aft_draft = mean_draft
                + ((displacement - calculated_displacement) / area_water_line)
                - (lbp / 2.0 + lcf) * ((lcg - lcb) / lmr);
            (nose_draft, aft_draft) = self.drafts(aft_draft, nose_draft);
            calculated_displacement = self
                .displacement
                .displacement_by_drafts(aft_draft, nose_draft)?;
            let err = (lcg - lcb).abs();
            let displacement_error = (displacement_tonnage
                - self.water_density.water_density() * calculated_displacement)
                .abs();
            info!("displ_error = {displacement_error}");
            info!("aft_draft = {}, nose_draft = {}", aft_draft, nose_draft);
            info!("'lcg - lcb' = {}", err);
            info!("lcg = {}, lcb = {}", lcg, lcb);
            info!("displacement = {displacement}, calculated_displacement = {calculated_displacement}");
            if self.trim_achieved(displacement_tonnage, calculated_displacement, lcg, lcb) {
                info!("Удифферентовка судна на тихой воде достигнута за {i} итераций.");
                return Ok((aft_draft, nose_draft));
            };
            lcb = self.lcb.lcb(aft_draft, nose_draft)?;
        }
        Err("Удифферентовка судна не достигнута из-за превышения максимального количества итераций.".to_string())
    }
}
